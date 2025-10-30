use clap::Parser;
use spectre::*;
use std::process;
use std::str::FromStr;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(name = "spectre")]
#[command(author = "Spectre Password Manager")]
#[command(version = VERSION)]
#[command(about = "Stateless password management solution", long_about = None)]
struct Args {
    /// Site name for which to generate a password
    site_name: Option<String>,

    /// User name (or -U to allow updating personal secret)
    #[arg(short = 'u', long, env = "SPECTRE_USERNAME")]
    user_name: Option<String>,

    /// Allow updating personal secret
    #[arg(short = 'U', long)]
    user_name_update: Option<String>,

    /// Read personal secret from file descriptor
    #[arg(short = 's', long)]
    secret_fd: Option<i32>,

    /// Personal secret (UNSAFE - for testing only)
    #[arg(short = 'S', long)]
    secret: Option<String>,

    /// Password template type
    #[arg(short = 't', long, default_value = "long")]
    result_type: String,

    /// Parameter value (login name, key bits, or personal password)
    #[arg(short = 'P', long)]
    result_param: Option<String>,

    /// Counter value
    #[arg(short = 'c', long, default_value = "1")]
    counter: u32,

    /// Algorithm version
    #[arg(short = 'a', long, env = "SPECTRE_ALGORITHM")]
    algorithm: Option<u32>,

    /// Key purpose (auth/ident/rec)
    #[arg(short = 'p', long, default_value = "auth")]
    purpose: String,

    /// Purpose-specific context
    #[arg(short = 'C', long)]
    context: Option<String>,

    /// File format (with fallback)
    #[arg(short = 'f', long, env = "SPECTRE_FORMAT")]
    format: Option<String>,

    /// File format (fixed, no fallback)
    #[arg(short = 'F', long)]
    format_fixed: Option<String>,

    /// Save file in redacted format
    #[arg(short = 'R', long, default_value = "1")]
    redacted: String,

    /// Increase verbosity
    #[arg(short = 'v', long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Decrease verbosity
    #[arg(short = 'q', long, action = clap::ArgAction::Count)]
    quiet: u8,

    /// Omit trailing newline
    #[arg(short = 'n', long)]
    no_newline: bool,
}

struct Operation {
    user_name: String,
    user_secret: String,
    site_name: String,
    result_type: SpectreResultType,
    result_param: Option<String>,
    counter: SpectreCounter,
    algorithm: SpectreAlgorithm,
    purpose: SpectreKeyPurpose,
    context: Option<String>,
    format: SpectreFormat,
    redacted: bool,
    no_newline: bool,
    allow_password_update: bool,
    verbosity: i8,
}

fn main() {
    let args = Args::parse();
    
    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    // Calculate verbosity level
    let verbosity = args.verbose as i8 - args.quiet as i8;
    
    // Determine if we're allowing password updates
    let (user_name_arg, allow_password_update) = if let Some(name) = args.user_name_update {
        (Some(name), true)
    } else {
        (args.user_name, false)
    };
    
    // Get user name
    let user_name = if let Some(name) = user_name_arg {
        name
    } else {
        prompt_line("Your full name:")?
    };
    
    if user_name.is_empty() {
        return Err(SpectreError::MissingField("user name".to_string()));
    }
    
    // Get user secret
    let mut user_secret = if let Some(secret) = args.secret {
        if verbosity >= 0 {
            eprintln!("Warning: Passing secrets via command-line is insecure!");
        }
        secret
    } else if let Some(fd) = args.secret_fd {
        read_fd(fd).map_err(SpectreError::Io)?
    } else {
        prompt_password("Your personal secret: ")?
    };
    
    if user_secret.is_empty() {
        return Err(SpectreError::MissingField("personal secret".to_string()));
    }
    
    // Get site name
    let site_name = if let Some(name) = args.site_name {
        name
    } else {
        prompt_line("Site Domain:")?
    };
    
    if site_name.is_empty() {
        return Err(SpectreError::MissingField("site name".to_string()));
    }
    
    // Parse result type
    let result_type = SpectreResultType::from_str(&args.result_type)?;
    
    // Parse key purpose
    let purpose = SpectreKeyPurpose::from_str(&args.purpose)?;
    
    // Determine file format
    let format_str = args.format_fixed.as_ref().or(args.format.as_ref());
    let format = if let Some(f) = format_str {
        SpectreFormat::parse(f)
            .ok_or_else(|| SpectreError::InvalidFileFormat(f.clone()))?
    } else {
        SPECTRE_FORMAT_DEFAULT
    };
    
    // Parse redacted flag
    let redacted = parse_bool(&args.redacted);
    
    // Determine algorithm version
    let algorithm = args.algorithm.unwrap_or(SPECTRE_ALGORITHM_CURRENT);
    if !(SPECTRE_ALGORITHM_FIRST..=SPECTRE_ALGORITHM_LAST).contains(&algorithm) {
        return Err(SpectreError::InvalidAlgorithm(algorithm));
    }
    
    // Counter is always valid since it's u32 and range is 0..u32::MAX
    // No validation needed
    
    let mut operation = Operation {
        user_name,
        user_secret: user_secret.clone(),
        site_name,
        result_type,
        result_param: args.result_param,
        counter: args.counter,
        algorithm,
        purpose,
        context: args.context,
        format,
        redacted,
        no_newline: args.no_newline,
        allow_password_update,
        verbosity,
    };
    
    // Execute the operation
    execute_operation(&mut operation)?;
    
    // Zero out sensitive data
    zero_string(&mut user_secret);
    zero_string(&mut operation.user_secret);
    
    Ok(())
}

fn execute_operation(op: &mut Operation) -> Result<()> {
    // Derive user key
    let user_key = spectre_user_key(&op.user_name, &op.user_secret, op.algorithm)?;
    
    // Generate identicon
    let identicon = spectre_identicon(&op.user_name, &op.user_secret)?;
    let identicon_render = spectre_identicon_render(identicon);
    
    // Load or create user file
    let file_path = if op.format != SpectreFormat::None {
        spectre_user_path(&op.user_name, op.format)
    } else {
        None
    };
    
    let mut user = if let Some(ref path) = file_path {
        match spectre_marshal_read(path) {
            Ok((_, Some(mut user))) => {
                // Authenticate user
                if let Err(SpectreError::UserSecretMismatch) = spectre_marshal_auth(&mut user, &op.user_secret) {
                    if op.allow_password_update {
                        eprintln!("Personal secret mismatch. Please confirm old secret to update.");
                        let old_secret = prompt_password("Old personal secret: ")?;
                        spectre_marshal_auth(&mut user, &old_secret)?;
                        
                        // Update to new secret
                        let new_key = spectre_user_key(&op.user_name, &op.user_secret, op.algorithm)?;
                        user.key_id = new_key.key_id;
                        user.identicon = identicon;
                    } else {
                        return Err(SpectreError::UserSecretMismatch);
                    }
                }
                user
            }
            _ => {
                // Create new user
                SpectreMarshalledUser::new(
                    op.user_name.clone(),
                    identicon,
                    user_key.key_id,
                    op.algorithm,
                )
            }
        }
    } else {
        // No file format, create ephemeral user
        SpectreMarshalledUser::new(
            op.user_name.clone(),
            identicon,
            user_key.key_id,
            op.algorithm,
        )
    };
    
    // Update redacted setting
    user.redacted = op.redacted;
    
    // Find or create site
    let mut site = if let Some(existing_site) = user.find_site(&op.site_name).cloned() {
        existing_site
    } else {
        SpectreMarshalledSite::new(
            op.site_name.clone(),
            op.result_type,
            op.counter,
            op.algorithm,
        )
    };
    
    // Update site settings if provided
    if op.result_type != SpectreResultType::None {
        match op.purpose {
            SpectreKeyPurpose::Authentication => {
                site.result_type = op.result_type;
                site.counter = op.counter;
            }
            SpectreKeyPurpose::Identification => {
                site.login_type = op.result_type;
            }
            SpectreKeyPurpose::Recovery => {
                // Handle recovery question
                let keyword = op.context.as_deref().unwrap_or("");
                let mut question = if let Some(q) = site.find_question(keyword).cloned() {
                    q
                } else {
                    SpectreMarshalledQuestion::new(keyword.to_string(), op.result_type)
                };
                question.question_type = op.result_type;
                site.add_question(question);
            }
        }
    }
    
    // Display info
    if op.verbosity >= 0 {
        eprintln!("{}'s {} for {}:", user.user_name, 
                  match op.purpose {
                      SpectreKeyPurpose::Authentication => "password",
                      SpectreKeyPurpose::Identification => "login",
                      SpectreKeyPurpose::Recovery => "answer",
                  },
                  site.site_name);
        eprintln!("[ {} ]", identicon_render);
    }
    
    // Handle stateful result types (encrypt if needed)
    if op.result_type.is_stateful() && op.result_param.is_some() {
        let plaintext = op.result_param.as_ref().unwrap();
        let state = spectre_site_state(
            &user_key,
            &op.site_name,
            op.result_type,
            plaintext,
            op.counter,
            op.purpose,
            op.context.as_deref(),
        )?;
        
        match op.purpose {
            SpectreKeyPurpose::Authentication => {
                site.result_state = Some(state.clone());
            }
            SpectreKeyPurpose::Identification => {
                site.login_state = Some(state.clone());
            }
            SpectreKeyPurpose::Recovery => {
                let keyword = op.context.as_deref().unwrap_or("");
                if let Some(question) = site.find_question_mut(keyword) {
                    question.state = Some(state.clone());
                }
            }
        }
        
        op.result_param = Some(state);
    }
    
    // Generate result
    let result = spectre_site_result(
        &user_key,
        &op.site_name,
        op.result_type,
        op.result_param.as_deref(),
        op.counter,
        op.purpose,
        op.context.as_deref(),
    )?;
    
    // Output result
    print!("{}", result);
    if !op.no_newline {
        println!();
    }
    
    // Update usage statistics
    site.uses += 1;
    site.last_used = chrono::Utc::now();
    user.last_used = chrono::Utc::now();
    
    // Save site back to user
    user.add_site(site);
    
    // Save user file if format is not None
    if let Some(path) = file_path
        && op.format != SpectreFormat::None {
            spectre_marshal_write(&path, op.format, &user)?;
            if op.verbosity >= 1 {
                eprintln!("Saved to: {}", path.display());
            }
        }
    
    Ok(())
}

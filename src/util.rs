/// Parse boolean from string
pub fn parse_bool(s: &str) -> bool {
    matches!(s, "1" | "true" | "yes" | "y" | "on")
}

/// Zero out a string's memory (security measure)
pub fn zero_string(s: &mut String) {
    unsafe {
        let bytes = s.as_bytes_mut();
        for byte in bytes.iter_mut() {
            *byte = 0;
        }
    }
    s.clear();
}

// CLI-only functions (not available in WASM)
#[cfg(feature = "cli")]
pub mod cli {
    use std::io::{self, Write};

    /// Prompt user for input
    pub fn prompt_line(prompt: &str) -> io::Result<String> {
        print!("{} ", prompt);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        Ok(input.trim().to_string())
    }

    /// Prompt user for password (hidden input)
    pub fn prompt_password(prompt: &str) -> io::Result<String> {
        rpassword::prompt_password(prompt)
    }

    /// Read from file descriptor (Unix only)
    #[cfg(unix)]
    pub fn read_fd(fd: i32) -> io::Result<String> {
        use std::os::unix::io::FromRawFd;
        use std::fs::File;
        use std::io::Read;
        
        let mut file = unsafe { File::from_raw_fd(fd) };
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        Ok(contents)
    }
}


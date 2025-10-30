use spectre::*;

/// Test case structure
#[derive(Debug)]
struct TestCase {
    user_name: &'static str,
    user_secret: &'static str,
    site_name: &'static str,
    result_type: SpectreResultType,
    counter: SpectreCounter,
    algorithm: SpectreAlgorithm,
    expected: &'static str,
}

impl TestCase {
    fn run(&self) -> Result<()> {
        let user_key = spectre_user_key(self.user_name, self.user_secret, self.algorithm)?;
        
        let result = spectre_site_result(
            &user_key,
            self.site_name,
            self.result_type,
            None,
            self.counter,
            SpectreKeyPurpose::Authentication,
            None,
        )?;
        
        assert_eq!(
            result, self.expected,
            "\nTest failed for:\n  User: {}\n  Site: {}\n  Type: {:?}\n  Expected: {}\n  Got: {}",
            self.user_name, self.site_name, self.result_type, self.expected, result
        );
        
        Ok(())
    }
}

#[test]
fn test_official_spectre_examples() {
    let test_cases = vec![
        // Test case from official Spectre site
        TestCase {
            user_name: "test",
            user_secret: "test",
            site_name: "masterpasswordapp.com",
            result_type: SpectreResultType::LongPassword,
            counter: 1,
            algorithm: SPECTRE_ALGORITHM_CURRENT,
            expected: "DolsZanoKipu3_", // Verified at https://spectre.app
        },
        
        // Robert Lee Mitchell test case
        TestCase {
            user_name: "Robert Lee Mitchell",
            user_secret: "banana colored duckling",
            site_name: "masterpasswordapp.com",
            result_type: SpectreResultType::LongPassword,
            counter: 1,
            algorithm: SPECTRE_ALGORITHM_CURRENT,
            expected: "Jejr5[RepuSosp", // Verified at https://spectre.app and from official JS test
        },
    ];
    
    for (i, test_case) in test_cases.iter().enumerate() {
        println!("Running test case {}...", i + 1);
        test_case.run().expect(&format!("Test case {} failed", i + 1));
        println!("✓ Test case {} passed", i + 1);
    }
}

#[test]
fn test_different_password_types() {
    let user_key = spectre_user_key("test", "test", SPECTRE_ALGORITHM_CURRENT)
        .expect("Failed to derive user key");
    
    // Test different password types
    let test_cases = vec![
        (SpectreResultType::MaximumSecurityPassword, "masterpasswordapp.com"),
        (SpectreResultType::LongPassword, "masterpasswordapp.com"),
        (SpectreResultType::MediumPassword, "masterpasswordapp.com"),
        (SpectreResultType::BasicPassword, "masterpasswordapp.com"),
        (SpectreResultType::ShortPassword, "masterpasswordapp.com"),
        (SpectreResultType::PIN, "masterpasswordapp.com"),
        (SpectreResultType::Name, "masterpasswordapp.com"),
        (SpectreResultType::Phrase, "masterpasswordapp.com"),
    ];
    
    for (result_type, site) in test_cases {
        let result = spectre_site_result(
            &user_key,
            site,
            result_type,
            None,
            1,
            SpectreKeyPurpose::Authentication,
            None,
        );
        
        assert!(result.is_ok(), "Failed to generate {:?} password", result_type);
        let password = result.unwrap();
        
        println!("{:?}: {}", result_type, password);
        
        // Verify password properties
        match result_type {
            SpectreResultType::PIN => {
                assert_eq!(password.len(), 4, "PIN should be 4 characters");
                assert!(password.chars().all(|c| c.is_numeric()), "PIN should only contain digits");
            }
            SpectreResultType::ShortPassword => {
                assert_eq!(password.len(), 4, "Short password should be 4 characters");
            }
            SpectreResultType::Name => {
                assert_eq!(password.len(), 9, "Name should be 9 characters");
                assert!(password.chars().all(|c| c.is_alphabetic()), "Name should only contain letters");
            }
            _ => {
                assert!(!password.is_empty(), "Password should not be empty");
            }
        }
    }
}

#[test]
fn test_counter_variation() {
    let user_key = spectre_user_key("test", "test", SPECTRE_ALGORITHM_CURRENT)
        .expect("Failed to derive user key");
    
    let mut passwords = vec![];
    
    // Generate passwords with different counters
    for counter in 1..=5 {
        let password = spectre_site_result(
            &user_key,
            "example.com",
            SpectreResultType::LongPassword,
            None,
            counter,
            SpectreKeyPurpose::Authentication,
            None,
        ).expect("Failed to generate password");
        
        println!("Counter {}: {}", counter, password);
        passwords.push(password);
    }
    
    // Verify all passwords are different
    for i in 0..passwords.len() {
        for j in (i + 1)..passwords.len() {
            assert_ne!(
                passwords[i], passwords[j],
                "Passwords with different counters should be different"
            );
        }
    }
}

#[test]
fn test_site_variation() {
    let user_key = spectre_user_key("test", "test", SPECTRE_ALGORITHM_CURRENT)
        .expect("Failed to derive user key");
    
    let sites = vec![
        "example.com",
        "github.com",
        "google.com",
        "amazon.com",
    ];
    
    let mut passwords = vec![];
    
    for site in &sites {
        let password = spectre_site_result(
            &user_key,
            site,
            SpectreResultType::LongPassword,
            None,
            1,
            SpectreKeyPurpose::Authentication,
            None,
        ).expect("Failed to generate password");
        
        println!("{}: {}", site, password);
        passwords.push(password);
    }
    
    // Verify all passwords are different
    for i in 0..passwords.len() {
        for j in (i + 1)..passwords.len() {
            assert_ne!(
                passwords[i], passwords[j],
                "Passwords for different sites should be different"
            );
        }
    }
}

#[test]
fn test_identicon_generation() {
    let test_cases = vec![
        ("test", "test"),
        ("Abdulrhman A", "nice work mate"),
        ("Alice", "secret123"),
    ];
    
    for (name, secret) in test_cases {
        let identicon = spectre_identicon(name, secret)
            .expect("Failed to generate identicon");
        
        let rendered = spectre_identicon_render(identicon);
        
        println!("{} / {}: {}", name, secret, rendered);
        
        // Verify identicon is consistent
        let identicon2 = spectre_identicon(name, secret)
            .expect("Failed to generate identicon");
        assert_eq!(identicon, identicon2, "Identicon should be deterministic");
    }
}

#[test]
fn test_key_purpose_variation() {
    let user_key = spectre_user_key("test", "test", SPECTRE_ALGORITHM_CURRENT)
        .expect("Failed to derive user key");
    
    let purposes = vec![
        SpectreKeyPurpose::Authentication,
        SpectreKeyPurpose::Identification,
        SpectreKeyPurpose::Recovery,
    ];
    
    for purpose in purposes {
        let result = spectre_site_result(
            &user_key,
            "example.com",
            SpectreResultType::LongPassword,
            None,
            1,
            purpose,
            None,
        ).expect("Failed to generate password");
        
        println!("{:?}: {}", purpose, result);
        assert!(!result.is_empty());
    }
}

#[test]
fn test_algorithm_versions() {
    let site = "example.com";
    let name = "test";
    let secret = "test";
    
    for version in SPECTRE_ALGORITHM_FIRST..=SPECTRE_ALGORITHM_LAST {
        let user_key = spectre_user_key(name, secret, version)
            .expect("Failed to derive user key");
        
        let password = spectre_site_result(
            &user_key,
            site,
            SpectreResultType::LongPassword,
            None,
            1,
            SpectreKeyPurpose::Authentication,
            None,
        ).expect("Failed to generate password");
        
        println!("Algorithm v{}: {}", version, password);
        assert!(!password.is_empty());
    }
}


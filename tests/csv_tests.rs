/// Test using CSV file with test vectors
/// 
/// This test reads test cases from tests/test_vectors.csv
/// Format: username,secret,site,type,counter,algorithm,expected
/// 
/// If expected is empty, the test will just print the result for manual verification
/// If expected is provided, the test will assert the result matches

use spectre::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct CsvTestCase {
    user_name: String,
    user_secret: String,
    site_name: String,
    result_type: String,
    counter: u32,
    algorithm: u32,
    expected: Option<String>,
}

impl CsvTestCase {
    fn from_csv_line(line: &str) -> Option<Self> {
        let fields: Vec<&str> = line.split(',').collect();
        
        if fields.len() < 7 {
            return None;
        }
        
        Some(CsvTestCase {
            user_name: fields[0].to_string(),
            user_secret: fields[1].to_string(),
            site_name: fields[2].to_string(),
            result_type: fields[3].to_string(),
            counter: fields[4].parse().ok()?,
            algorithm: fields[5].parse().ok()?,
            expected: if fields[6].is_empty() {
                None
            } else {
                Some(fields[6].to_string())
            },
        })
    }
    
    fn run(&self) -> Result<String> {
        let result_type = match self.result_type.as_str() {
            "long" => SpectreResultType::LongPassword,
            "medium" => SpectreResultType::MediumPassword,
            "short" => SpectreResultType::ShortPassword,
            "basic" => SpectreResultType::BasicPassword,
            "maximum" => SpectreResultType::MaximumSecurityPassword,
            "pin" => SpectreResultType::PIN,
            "name" => SpectreResultType::Name,
            "phrase" => SpectreResultType::Phrase,
            _ => return Err(SpectreError::InvalidResultType(self.result_type.clone())),
        };
        
        let user_key = spectre_user_key(&self.user_name, &self.user_secret, self.algorithm)?;
        
        let result = spectre_site_result(
            &user_key,
            &self.site_name,
            result_type,
            None,
            self.counter,
            SpectreKeyPurpose::Authentication,
            None,
        )?;
        
        Ok(result)
    }
}

#[test]
fn test_from_csv_file() {
    let csv_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/test_vectors.csv");
    
    let file = File::open(&csv_path).expect("Failed to open test_vectors.csv");
    let reader = BufReader::new(file);
    
    let mut passed = 0;
    let mut failed = 0;
    let mut manual = 0;
    
    println!("\n{}\n  Running CSV Test Vectors\n{}\n", "=".repeat(70), "=".repeat(70));
    
    for (line_num, line) in reader.lines().enumerate() {
        let line = line.expect("Failed to read line");
        
        // Skip header line
        if line_num == 0 {
            continue;
        }
        
        // Skip empty lines and comments
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }
        
        let test_case = match CsvTestCase::from_csv_line(&line) {
            Some(tc) => tc,
            None => {
                println!("⚠️  Line {}: Skipping invalid format", line_num + 1);
                continue;
            }
        };
        
        match test_case.run() {
            Ok(result) => {
                if let Some(expected) = &test_case.expected {
                    if &result == expected {
                        println!("✓ Line {}: {} @ {} = {}", 
                            line_num + 1, 
                            test_case.user_name, 
                            test_case.site_name, 
                            result
                        );
                        passed += 1;
                    } else {
                        println!("✗ Line {}: {} @ {}", 
                            line_num + 1, 
                            test_case.user_name, 
                            test_case.site_name
                        );
                        println!("  Expected: {}", expected);
                        println!("  Got:      {}", result);
                        failed += 1;
                    }
                } else {
                    println!("ℹ Line {}: {} @ {} = {} (no expected value)", 
                        line_num + 1, 
                        test_case.user_name, 
                        test_case.site_name, 
                        result
                    );
                    manual += 1;
                }
            }
            Err(e) => {
                println!("✗ Line {}: Error - {}", line_num + 1, e);
                failed += 1;
            }
        }
    }
    
    println!("\n{}", "=".repeat(70));
    println!("  Results: {} passed, {} failed, {} manual verification", passed, failed, manual);
    println!("{}\n", "=".repeat(70));
    
    if failed > 0 {
        panic!("{} test(s) failed", failed);
    }
}


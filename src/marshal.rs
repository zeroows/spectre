use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use crate::error::{Result, SpectreError};
use crate::models::*;
use crate::algorithm::{spectre_user_key, spectre_identicon};

/// Read a marshalled user file
pub fn spectre_marshal_read(file_path: &PathBuf) -> Result<(SpectreMarshalledFile, Option<SpectreMarshalledUser>)> {
    if !file_path.exists() {
        return Ok((
            SpectreMarshalledFile::new(SPECTRE_FORMAT_DEFAULT, true),
            None,
        ));
    }

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Try to parse as JSON first
    match serde_json::from_str::<SpectreMarshalledUser>(&contents) {
        Ok(user) => {
            let file_meta = SpectreMarshalledFile::new(SpectreFormat::JSON, user.redacted);
            Ok((file_meta, Some(user)))
        }
        Err(_) => {
            // Try to parse as flat format
            parse_flat_format(&contents)
        }
    }
}

/// Write a marshalled user file
pub fn spectre_marshal_write(
    file_path: &PathBuf,
    format: SpectreFormat,
    user: &SpectreMarshalledUser,
) -> Result<()> {
    // Ensure directory exists
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let contents = match format {
        SpectreFormat::JSON => {
            serde_json::to_string_pretty(user)?
        }
        SpectreFormat::Flat => {
            write_flat_format(user)?
        }
        SpectreFormat::None => {
            return Ok(());
        }
    };

    let mut file = File::create(file_path)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}

/// Parse flat format (simplified version)
fn parse_flat_format(_contents: &str) -> Result<(SpectreMarshalledFile, Option<SpectreMarshalledUser>)> {
    // This is a simplified parser - the full implementation would be more complex
    // For now, return an error to encourage JSON format
    Err(SpectreError::InvalidFileFormat("Flat format parsing not fully implemented. Please use JSON format.".to_string()))
}

/// Write flat format (simplified version)
fn write_flat_format(_user: &SpectreMarshalledUser) -> Result<String> {
    // This is a simplified writer - the full implementation would be more complex
    Err(SpectreError::InvalidFileFormat("Flat format writing not fully implemented. Please use JSON format.".to_string()))
}

/// Authenticate user with their secret
pub fn spectre_marshal_auth(
    user: &mut SpectreMarshalledUser,
    user_secret: &str,
) -> Result<()> {
    let user_key = spectre_user_key(&user.user_name, user_secret, user.algorithm)?;
    
    // Verify key ID matches
    if user.key_id != user_key.key_id {
        return Err(SpectreError::UserSecretMismatch);
    }
    
    // Update identicon
    let identicon = spectre_identicon(&user.user_name, user_secret)?;
    user.identicon = identicon;
    
    Ok(())
}

/// Get the default user file path
pub fn spectre_user_path(user_name: &str, format: SpectreFormat) -> Option<PathBuf> {
    let home_dir = dirs::home_dir()?;
    let spectre_dir = home_dir.join(".spectre.d");
    
    let extension = format.extension();
    if extension.is_empty() {
        return None;
    }
    
    let filename = format!("{}.{}", user_name, extension);
    Some(spectre_dir.join(filename))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    #[test]
    fn test_user_path() {
        let path = spectre_user_path("testuser", SpectreFormat::JSON);
        assert!(path.is_some());
        assert!(path.unwrap().to_string_lossy().contains(".spectre.d"));
    }

    #[test]
    fn test_json_serialization() {
        let user = SpectreMarshalledUser::new(
            "testuser".to_string(),
            [0, 1, 2, 3],
            [0u8; 32],
            SPECTRE_ALGORITHM_CURRENT,
        );
        
        let json = serde_json::to_string(&user);
        assert!(json.is_ok());
    }
}


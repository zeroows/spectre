use hmac::{Hmac, Mac};
use sha2::{Sha256, Digest};
use scrypt::{scrypt, Params};
use crate::error::{Result, SpectreError};
use crate::types::*;

type HmacSha256 = Hmac<Sha256>;

const SPECTRE_N: u32 = 32768;
const SPECTRE_R: u32 = 8;
const SPECTRE_P: u32 = 2;
const SPECTRE_DK_LEN: usize = 64;

#[derive(Clone)]
pub struct SpectreUserKey {
    pub key_id: [u8; 32],
    pub key_data: Vec<u8>,
    pub algorithm: SpectreAlgorithm,
}

/// Derive a user key from the user's name and secret
pub fn spectre_user_key(
    user_name: &str,
    user_secret: &str,
    algorithm: SpectreAlgorithm,
) -> Result<SpectreUserKey> {
    if !(SPECTRE_ALGORITHM_FIRST..=SPECTRE_ALGORITHM_LAST).contains(&algorithm) {
        return Err(SpectreError::InvalidAlgorithm(algorithm));
    }

    // Create salt: "com.lyndir.masterpassword" + user_name length + user_name
    let scope = b"com.lyndir.masterpassword";
    let name_bytes = user_name.as_bytes();
    let name_len = (name_bytes.len() as u32).to_be_bytes();
    
    let mut salt = Vec::with_capacity(scope.len() + 4 + name_bytes.len());
    salt.extend_from_slice(scope);
    salt.extend_from_slice(&name_len);
    salt.extend_from_slice(name_bytes);

    // Derive key using scrypt
    let mut key_data = vec![0u8; SPECTRE_DK_LEN];
    let params = Params::new(
        (SPECTRE_N as f64).log2() as u8,
        SPECTRE_R,
        SPECTRE_P,
        SPECTRE_DK_LEN,
    ).map_err(|_| SpectreError::KeyDerivationFailed)?;
    
    scrypt(user_secret.as_bytes(), &salt, &params, &mut key_data)
        .map_err(|_| SpectreError::KeyDerivationFailed)?;

    // Generate key ID (SHA256 of the key)
    let mut hasher = Sha256::new();
    hasher.update(&key_data);
    let key_id_vec = hasher.finalize();
    let mut key_id = [0u8; 32];
    key_id.copy_from_slice(&key_id_vec);

    Ok(SpectreUserKey {
        key_id,
        key_data,
        algorithm,
    })
}

/// Generate a site password
pub fn spectre_site_result(
    user_key: &SpectreUserKey,
    site_name: &str,
    result_type: SpectreResultType,
    result_param: Option<&str>,
    key_counter: SpectreCounter,
    key_purpose: SpectreKeyPurpose,
    key_context: Option<&str>,
) -> Result<String> {
    // Derive site key
    let site_key = spectre_site_key(
        user_key,
        site_name,
        key_counter,
        key_purpose,
        key_context,
    )?;

    // For stateful types, handle differently
    if result_type.is_stateful() {
        if let Some(state) = result_param {
            return Ok(state.to_string());
        }
        return Err(SpectreError::PasswordGenerationFailed);
    }

    // Generate password from template
    let templates = result_type.template();
    if templates.is_empty() {
        return Err(SpectreError::PasswordGenerationFailed);
    }

    // Select template based on seed
    let template_index = site_key[0] as usize % templates.len();
    let template = templates[template_index];

    // Generate password from template
    let mut password = String::new();
    let mut seed_index = 1; // Start at 1 since 0 was used for template selection
    for template_char in template.chars() {
        let char_class = char_class_for_template(template_char);
        if char_class.is_empty() {
            password.push(template_char);
        } else {
            let char_index = site_key[seed_index % site_key.len()] as usize % char_class.len();
            password.push(char_class[char_index]);
            seed_index += 1;
        }
    }

    Ok(password)
}

/// Derive a site-specific key
fn spectre_site_key(
    user_key: &SpectreUserKey,
    site_name: &str,
    key_counter: SpectreCounter,
    key_purpose: SpectreKeyPurpose,
    key_context: Option<&str>,
) -> Result<Vec<u8>> {
    let scope: &[u8] = match key_purpose {
        SpectreKeyPurpose::Authentication => b"com.lyndir.masterpassword",
        SpectreKeyPurpose::Identification => b"com.lyndir.masterpassword.login",
        SpectreKeyPurpose::Recovery => b"com.lyndir.masterpassword.answer",
    };

    let site_bytes = site_name.as_bytes();
    let site_len = (site_bytes.len() as u32).to_be_bytes();

    let mut salt = Vec::new();
    salt.extend_from_slice(scope);
    salt.extend_from_slice(&site_len);
    salt.extend_from_slice(site_bytes);
    salt.extend_from_slice(&key_counter.to_be_bytes());

    if let Some(context) = key_context {
        if !context.is_empty() {
            let context_bytes = context.as_bytes();
            let context_len = (context_bytes.len() as u32).to_be_bytes();
            salt.extend_from_slice(&context_len);
            salt.extend_from_slice(context_bytes);
        }
    }

    // Use HMAC-SHA256 to derive site key
    let mut mac = HmacSha256::new_from_slice(&user_key.key_data)
        .map_err(|_| SpectreError::KeyDerivationFailed)?;
    mac.update(&salt);
    let result = mac.finalize();
    
    Ok(result.into_bytes().to_vec())
}

/// Generate an identicon for a user
pub fn spectre_identicon(user_name: &str, user_secret: &str) -> Result<[u8; 4]> {
    let user_key = spectre_user_key(user_name, user_secret, SPECTRE_ALGORITHM_CURRENT)?;
    
    let mut identicon = [0u8; 4];
    identicon.copy_from_slice(&user_key.key_data[0..4]);
    
    Ok(identicon)
}

/// Render an identicon as a visual string
pub fn spectre_identicon_render(identicon: [u8; 4]) -> String {
    const COLORS: &[&str] = &[
        "🔴", "🟠", "🟡", "🟢", "🔵", "🟣", "🟤", "⚫",
        "🔴", "🟠", "🟡", "🟢", "🔵", "🟣", "🟤", "⚪",
    ];
    
    let mut result = String::new();
    for &byte in &identicon {
        let index = (byte % 16) as usize;
        result.push_str(COLORS[index]);
    }
    
    result
}

/// Encrypt a personal password (stateful)
pub fn spectre_site_state(
    user_key: &SpectreUserKey,
    site_name: &str,
    _result_type: SpectreResultType,
    plaintext: &str,
    key_counter: SpectreCounter,
    key_purpose: SpectreKeyPurpose,
    key_context: Option<&str>,
) -> Result<String> {
    // For now, just base64 encode (in production, should use proper encryption)
    // This is a simplified version - the C implementation uses proper AES encryption
    use std::fmt::Write;
    
    let site_key = spectre_site_key(
        user_key,
        site_name,
        key_counter,
        key_purpose,
        key_context,
    )?;
    
    // XOR encrypt (simplified - should use AES in production)
    let mut encrypted = Vec::new();
    for (i, &byte) in plaintext.as_bytes().iter().enumerate() {
        encrypted.push(byte ^ site_key[i % site_key.len()]);
    }
    
    // Hex encode
    let mut result = String::new();
    for byte in encrypted {
        write!(&mut result, "{:02x}", byte).unwrap();
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_key_derivation() {
        let result = spectre_user_key("Abdulrhman A", "nice work mate", SPECTRE_ALGORITHM_CURRENT);
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_generation() {
        let user_key = spectre_user_key("Abdulrhman A", "nice work mate", SPECTRE_ALGORITHM_CURRENT).unwrap();
        let password = spectre_site_result(
            &user_key,
            "masterpasswordapp.com",
            SpectreResultType::LongPassword,
            None,
            1,
            SpectreKeyPurpose::Authentication,
            None,
        );
        assert!(password.is_ok());
    }
}


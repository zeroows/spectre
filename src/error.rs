use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpectreError {
    #[error("Invalid algorithm version: {0}")]
    InvalidAlgorithm(u32),
    
    #[error("Invalid result type: {0}")]
    InvalidResultType(String),
    
    #[error("Invalid key purpose: {0}")]
    InvalidKeyPurpose(String),
    
    #[error("Invalid counter value: {0}")]
    InvalidCounter(i64),
    
    #[error("Invalid file format: {0}")]
    InvalidFileFormat(String),
    
    #[error("User key derivation failed")]
    KeyDerivationFailed,
    
    #[error("Password generation failed")]
    PasswordGenerationFailed,
    
    #[error("Encryption failed")]
    EncryptionFailed,
    
    #[error("Decryption failed")]
    DecryptionFailed,
    
    #[error("User secret mismatch")]
    UserSecretMismatch,
    
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Missing required field: {0}")]
    MissingField(String),
}

pub type Result<T> = std::result::Result<T, SpectreError>;


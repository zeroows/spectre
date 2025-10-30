use serde::{Deserialize, Serialize};
use std::str::FromStr;
use crate::error::{Result, SpectreError};

pub const SPECTRE_ALGORITHM_FIRST: u32 = 0;
pub const SPECTRE_ALGORITHM_CURRENT: u32 = 3;
pub const SPECTRE_ALGORITHM_LAST: u32 = 3;

pub const SPECTRE_COUNTER_DEFAULT: u32 = 1;
pub const SPECTRE_COUNTER_INITIAL: u32 = 0;
pub const SPECTRE_COUNTER_FIRST: u32 = 0;
pub const SPECTRE_COUNTER_LAST: u32 = u32::MAX;

pub type SpectreAlgorithm = u32;
pub type SpectreCounter = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpectreKeyPurpose {
    Authentication,
    Identification,
    Recovery,
}

impl SpectreKeyPurpose {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Authentication => "authentication",
            Self::Identification => "identification",
            Self::Recovery => "recovery",
        }
    }
}

impl FromStr for SpectreKeyPurpose {
    type Err = SpectreError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "a" | "auth" | "authentication" => Ok(Self::Authentication),
            "i" | "ident" | "identification" => Ok(Self::Identification),
            "r" | "rec" | "recovery" => Ok(Self::Recovery),
            _ => Err(SpectreError::InvalidKeyPurpose(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[repr(u32)]
pub enum SpectreResultType {
    // Template-based results
    MaximumSecurityPassword = 0x00010000,
    LongPassword = 0x00010001,
    MediumPassword = 0x00010002,
    BasicPassword = 0x00010004,
    ShortPassword = 0x00010003,
    PIN = 0x00010005,
    Name = 0x00010006,
    Phrase = 0x00010007,
    
    // Stateful results
    PersonalPassword = 0x00020000,
    DeriveKey = 0x00020001,
    
    #[default]
    None = 0x0,
}

impl SpectreResultType {
    pub fn short_name(&self) -> &'static str {
        match self {
            Self::MaximumSecurityPassword => "maximum",
            Self::LongPassword => "long",
            Self::MediumPassword => "medium",
            Self::BasicPassword => "basic",
            Self::ShortPassword => "short",
            Self::PIN => "pin",
            Self::Name => "name",
            Self::Phrase => "phrase",
            Self::DeriveKey => "key",
            Self::PersonalPassword => "personal",
            Self::None => "none",
        }
    }

    pub fn is_stateful(&self) -> bool {
        matches!(self, Self::PersonalPassword | Self::DeriveKey)
    }

    pub fn template(&self) -> &'static [&'static str] {
        match self {
            Self::MaximumSecurityPassword => &[
                "anoxxxxxxxxxxxxxxxxx",
                "axxxxxxxxxxxxxxxxxno",
            ],
            Self::LongPassword => &[
                "CvcvnoCvcvCvcv",
                "CvcvCvcvnoCvcv",
                "CvcvCvcvCvcvno",
                "CvccnoCvcvCvcv",
                "CvccCvcvnoCvcv",
                "CvccCvcvCvcvno",
                "CvcvnoCvccCvcv",
                "CvcvCvccnoCvcv",
                "CvcvCvccCvcvno",
                "CvcvnoCvcvCvcc",
                "CvcvCvcvnoCvcc",
                "CvcvCvcvCvccno",
                "CvccnoCvccCvcv",
                "CvccCvccnoCvcv",
                "CvccCvccCvcvno",
                "CvcvnoCvccCvcc",
                "CvcvCvccnoCvcc",
                "CvcvCvccCvccno",
                "CvccnoCvcvCvcc",
                "CvccCvcvnoCvcc",
                "CvccCvcvCvccno",
            ],
            Self::MediumPassword => &[
                "CvcnoCvc",
                "CvcCvcno",
            ],
            Self::BasicPassword => &[
                "aaanaaan",
                "aannaaan",
                "aaannaaa",
            ],
            Self::ShortPassword => &[
                "Cvcn",
            ],
            Self::PIN => &[
                "nnnn",
            ],
            Self::Name => &[
                "cvccvcvcv",
            ],
            Self::Phrase => &[
                "cvcc cvc cvccvcv cvc",
                "cvc cvccvcvcv cvcv",
                "cv cvccv cvc cvcvccv",
            ],
            _ => &[],
        }
    }
}

impl FromStr for SpectreResultType {
    type Err = SpectreError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "x" | "max" | "maximum" => Ok(Self::MaximumSecurityPassword),
            "l" | "long" => Ok(Self::LongPassword),
            "m" | "medium" => Ok(Self::MediumPassword),
            "b" | "basic" => Ok(Self::BasicPassword),
            "s" | "short" => Ok(Self::ShortPassword),
            "i" | "pin" => Ok(Self::PIN),
            "n" | "name" => Ok(Self::Name),
            "p" | "phrase" => Ok(Self::Phrase),
            "K" | "key" => Ok(Self::DeriveKey),
            "P" | "personal" => Ok(Self::PersonalPassword),
            _ => Err(SpectreError::InvalidResultType(s.to_string())),
        }
    }
}

pub const SPECTRE_RESULT_DEFAULT_RESULT: SpectreResultType = SpectreResultType::LongPassword;

// Character classes for templates - based on official Spectre algorithm specification
pub fn char_class_for_template(c: char) -> &'static [char] {
    match c {
        'V' => &['A', 'E', 'I', 'O', 'U'],
        'C' => &['B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Y', 'Z'],
        'v' => &['a', 'e', 'i', 'o', 'u'],
        'c' => &['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z'],
        'A' => &['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'],
        'a' => &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'],
        'n' => &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
        // Official Spectre symbol set - order matters!
        // From the reference implementation: "@&%?,=[]_:-+*$#!'^~;()/."
        'o' => &['@', '&', '%', '?', ',', '=', '[', ']', '_', ':', '-', '+', '*', '$', '#', '!', '\'', '^', '~', ';', '(', ')', '/', '.'],
        'x' => &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'],
        ' ' => &[' '],
        _ => &[],
    }
}


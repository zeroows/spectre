use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectreMarshalledFile {
    pub format: SpectreFormat,
    pub redacted: bool,
    pub date: DateTime<Utc>,
    #[serde(skip)]
    pub error: SpectreMarshalError,
}

#[derive(Debug, Clone)]
pub struct SpectreMarshalError {
    pub error_type: SpectreMarshalErrorType,
    pub message: String,
}

impl Default for SpectreMarshalError {
    fn default() -> Self {
        Self {
            error_type: SpectreMarshalErrorType::Success,
            message: String::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpectreMarshalErrorType {
    Success,
    ErrorUserSecret,
    ErrorFormat,
    ErrorMissing,
    ErrorIllegal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpectreFormat {
    None,
    Flat,
    JSON,
}

impl SpectreFormat {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "n" | "none" => Some(Self::None),
            "f" | "flat" => Some(Self::Flat),
            "j" | "json" => Some(Self::JSON),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Flat => "flat",
            Self::JSON => "json",
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Flat => "mpsites",
            Self::JSON => "json",
        }
    }
}

pub const SPECTRE_FORMAT_DEFAULT: SpectreFormat = SpectreFormat::JSON;
pub const SPECTRE_FORMAT_FIRST: SpectreFormat = SpectreFormat::Flat;
pub const SPECTRE_FORMAT_LAST: SpectreFormat = SpectreFormat::JSON;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectreMarshalledUser {
    pub user_name: String,
    pub identicon: [u8; 4],
    pub key_id: [u8; 32],
    pub algorithm: SpectreAlgorithm,
    pub redacted: bool,
    
    #[serde(default)]
    pub login_type: SpectreResultType,
    #[serde(default)]
    pub login_state: Option<String>,
    
    pub last_used: DateTime<Utc>,
    
    #[serde(default)]
    pub sites: Vec<SpectreMarshalledSite>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectreMarshalledSite {
    pub site_name: String,
    pub result_type: SpectreResultType,
    pub result_state: Option<String>,
    pub counter: SpectreCounter,
    pub algorithm: SpectreAlgorithm,
    
    #[serde(default)]
    pub login_type: SpectreResultType,
    #[serde(default)]
    pub login_state: Option<String>,
    
    #[serde(default)]
    pub url: Option<String>,
    
    pub uses: u32,
    pub last_used: DateTime<Utc>,
    
    #[serde(default)]
    pub questions: Vec<SpectreMarshalledQuestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpectreMarshalledQuestion {
    pub keyword: String,
    pub question_type: SpectreResultType,
    pub state: Option<String>,
}

impl SpectreMarshalledFile {
    pub fn new(format: SpectreFormat, redacted: bool) -> Self {
        Self {
            format,
            redacted,
            date: Utc::now(),
            error: SpectreMarshalError::default(),
        }
    }
}

impl SpectreMarshalledUser {
    pub fn new(
        user_name: String,
        identicon: [u8; 4],
        key_id: [u8; 32],
        algorithm: SpectreAlgorithm,
    ) -> Self {
        Self {
            user_name,
            identicon,
            key_id,
            algorithm,
            redacted: true,
            login_type: SpectreResultType::None,
            login_state: None,
            last_used: Utc::now(),
            sites: Vec::new(),
        }
    }
    
    pub fn add_site(&mut self, site: SpectreMarshalledSite) {
        // Check if site already exists
        if let Some(existing) = self.sites.iter_mut().find(|s| s.site_name == site.site_name) {
            *existing = site;
        } else {
            self.sites.push(site);
        }
    }
    
    pub fn find_site(&self, site_name: &str) -> Option<&SpectreMarshalledSite> {
        self.sites.iter().find(|s| s.site_name == site_name)
    }
    
    pub fn find_site_mut(&mut self, site_name: &str) -> Option<&mut SpectreMarshalledSite> {
        self.sites.iter_mut().find(|s| s.site_name == site_name)
    }
}

impl SpectreMarshalledSite {
    pub fn new(
        site_name: String,
        result_type: SpectreResultType,
        counter: SpectreCounter,
        algorithm: SpectreAlgorithm,
    ) -> Self {
        Self {
            site_name,
            result_type,
            result_state: None,
            counter,
            algorithm,
            login_type: SpectreResultType::None,
            login_state: None,
            url: None,
            uses: 0,
            last_used: Utc::now(),
            questions: Vec::new(),
        }
    }
    
    pub fn add_question(&mut self, question: SpectreMarshalledQuestion) {
        // Check if question already exists
        if let Some(existing) = self.questions.iter_mut().find(|q| q.keyword == question.keyword) {
            *existing = question;
        } else {
            self.questions.push(question);
        }
    }
    
    pub fn find_question(&self, keyword: &str) -> Option<&SpectreMarshalledQuestion> {
        self.questions.iter().find(|q| q.keyword == keyword)
    }
    
    pub fn find_question_mut(&mut self, keyword: &str) -> Option<&mut SpectreMarshalledQuestion> {
        self.questions.iter_mut().find(|q| q.keyword == keyword)
    }
}

impl SpectreMarshalledQuestion {
    pub fn new(keyword: String, question_type: SpectreResultType) -> Self {
        Self {
            keyword,
            question_type,
            state: None,
        }
    }
}


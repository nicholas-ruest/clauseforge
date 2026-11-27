use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AmendmentRequest {
    pub supplier: String,
    pub amendment: String,
    pub affected_controls: u8,
    pub cure_days: u32,
    pub reviewer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StageResult {
    pub stage: String,
    pub score: f64,
    pub message: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum DomainError {
    #[error("missing field: {0}")]
    Missing(&'static str),
    #[error("value out of range: {0}")]
    Range(&'static str),
    #[error("policy denied: {0}")]
    Denied(String),
    #[error("upstream unavailable: {0}")]
    Upstream(String),
    #[error("evidence integrity failure: {0}")]
    Integrity(String),
    #[error("storage failure: {0}")]
    Storage(String),
}

impl AmendmentRequest {
    pub fn validate_identity(&self) -> Result<(), DomainError> {
        if self.supplier.trim().is_empty() {
            return Err(DomainError::Missing("supplier"));
        }
        if self.amendment.trim().is_empty() {
            return Err(DomainError::Missing("amendment"));
        }
        if self.reviewer.trim().is_empty() {
            return Err(DomainError::Missing("reviewer"));
        }
        if self.affected_controls > 100 {
            return Err(DomainError::Range("affected_controls"));
        }
        if self.cure_days == 0 || self.cure_days > 365 {
            return Err(DomainError::Range("cure_days"));
        }
        Ok(())
    }

    pub fn stable_key(&self) -> String {
        format!(
            "{}:{}:{}",
            self.supplier.trim().to_lowercase(),
            self.amendment.trim().to_lowercase(),
            self.reviewer.trim().to_lowercase()
        )
    }
}

pub trait Stage {
    fn name(&self) -> &'static str;
    fn run(&self, request: &AmendmentRequest) -> Result<StageResult, DomainError>;
}

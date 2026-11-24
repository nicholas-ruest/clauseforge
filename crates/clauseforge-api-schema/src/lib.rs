use clauseforge_kernel::AmendmentRequest;
use serde::{Deserialize, Serialize};

pub const API_VERSION: &str = "v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubmitAmendment {
    pub supplier: String,
    pub amendment: String,
    pub affected_controls: u8,
    pub cure_days: u32,
}

impl SubmitAmendment {
    pub fn into_domain(self, reviewer: String) -> AmendmentRequest {
        AmendmentRequest {
            supplier: self.supplier,
            amendment: self.amendment,
            affected_controls: self.affected_controls,
            cure_days: self.cure_days,
            reviewer,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AmendmentResponse {
    pub id: String,
    pub correlation_id: String,
    pub status: String,
    pub risk_score: f64,
    pub stages: Vec<String>,
    pub evidence_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub correlation_id: String,
}

pub fn schema_fingerprint() -> String {
    "clauseforge-api-v1-2026-11-23".into()
}

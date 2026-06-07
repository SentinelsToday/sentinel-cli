use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CloudError {
    #[error("http error: {0}")]
    Http(#[from] Box<ureq::Error>),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("server returned {status}: {body}")]
    Status { status: u16, body: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceRecord {
    pub did: String,
    pub public_key_hex: String,
    pub registered_at: String,
    pub firmware_verified: bool,
    pub heartbeat_count: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrustScore {
    pub score: u8,
    pub level: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: String,
    pub robot_id: String,
    pub action: String,
    pub details: serde_json::Value,
    pub timestamp: String,
    pub hash: String,
    pub previous_hash: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditPage {
    pub entries: Vec<AuditEntry>,
}

pub struct Cloud {
    base_url: String,
}

impl Cloud {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
        }
    }

    pub fn get_device(&self, did: &str) -> Result<DeviceRecord, CloudError> {
        let res = check(ureq::get(&format!("{}/v1/devices/{}", self.base_url, did)).call())?;
        Ok(res.into_json::<DeviceRecord>()?)
    }

    pub fn get_trust(&self, did: &str) -> Result<TrustScore, CloudError> {
        let res = check(ureq::get(&format!("{}/v1/devices/{}/trust", self.base_url, did)).call())?;
        Ok(res.into_json::<TrustScore>()?)
    }

    pub fn get_audit(&self, robot_id: &str) -> Result<AuditPage, CloudError> {
        let res = check(ureq::get(&format!("{}/v1/audit/{}", self.base_url, robot_id)).call())?;
        Ok(res.into_json::<AuditPage>()?)
    }
}

fn check(result: Result<ureq::Response, ureq::Error>) -> Result<ureq::Response, CloudError> {
    match result {
        Ok(r) => Ok(r),
        Err(ureq::Error::Status(status, response)) => Err(CloudError::Status {
            status,
            body: response.into_string().unwrap_or_default(),
        }),
        Err(e) => Err(CloudError::Http(Box::new(e))),
    }
}

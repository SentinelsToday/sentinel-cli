use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::cloud::AuditEntry;

#[derive(Debug, Error)]
pub enum AuditVerifyError {
    #[error("hash mismatch at index {index} (entry {id})")]
    HashMismatch { index: usize, id: String },
    #[error("broken previous-hash link at index {0}")]
    BrokenLink(usize),
}

/// Recompute the hash chain and ensure each entry's `hash` and `previous_hash`
/// fields line up. This implements the same pre-image used by sentinel-cloud
/// and sentinel-core: `id|robot|action|details_json|ts|prev`.
pub fn verify(entries: &[AuditEntry]) -> Result<(), AuditVerifyError> {
    let mut previous: Option<&str> = None;
    for (i, e) in entries.iter().enumerate() {
        let preimage = format!(
            "{}|{}|{}|{}|{}|{}",
            e.id,
            e.robot_id,
            e.action,
            serde_json::to_string(&e.details).unwrap_or_default(),
            e.timestamp,
            previous.unwrap_or("")
        );
        let expected = hex::encode(Sha256::digest(preimage.as_bytes()));
        if expected != e.hash {
            return Err(AuditVerifyError::HashMismatch {
                index: i,
                id: e.id.clone(),
            });
        }
        if e.previous_hash.as_deref() != previous {
            return Err(AuditVerifyError::BrokenLink(i));
        }
        previous = Some(&e.hash);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(
        id: &str,
        robot: &str,
        action: &str,
        details: serde_json::Value,
        ts: &str,
        prev: Option<&str>,
    ) -> AuditEntry {
        let preimage = format!(
            "{}|{}|{}|{}|{}|{}",
            id,
            robot,
            action,
            serde_json::to_string(&details).unwrap_or_default(),
            ts,
            prev.unwrap_or("")
        );
        let hash = hex::encode(Sha256::digest(preimage.as_bytes()));
        AuditEntry {
            id: id.into(),
            robot_id: robot.into(),
            action: action.into(),
            details,
            timestamp: ts.into(),
            hash,
            previous_hash: prev.map(String::from),
        }
    }

    #[test]
    fn clean_chain_verifies() {
        let e1 = make_entry(
            "a",
            "r",
            "boot",
            serde_json::json!({}),
            "2026-01-01T00:00:00Z",
            None,
        );
        let e2 = make_entry(
            "b",
            "r",
            "telemetry",
            serde_json::json!({"v": 1}),
            "2026-01-01T00:01:00Z",
            Some(&e1.hash),
        );
        verify(&[e1, e2]).unwrap();
    }

    #[test]
    fn tampered_chain_is_rejected() {
        let e1 = make_entry(
            "a",
            "r",
            "boot",
            serde_json::json!({}),
            "2026-01-01T00:00:00Z",
            None,
        );
        let mut bad = e1.clone();
        bad.details = serde_json::json!({"tampered": true});
        let err = verify(&[bad]).unwrap_err();
        assert!(matches!(err, AuditVerifyError::HashMismatch { .. }));
    }
}

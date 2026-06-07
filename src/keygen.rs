use ed25519_dalek::SigningKey;
use rand_core::OsRng;
use sha2::{Digest, Sha256};

pub struct GeneratedKey {
    pub did: String,
    pub secret_hex: String,
    pub public_key_hex: String,
}

pub fn generate() -> GeneratedKey {
    let signing = SigningKey::generate(&mut OsRng);
    let pk = signing.verifying_key();
    let did = format!(
        "did:sentinel:{}",
        hex::encode(Sha256::digest(pk.as_bytes()))
    );
    GeneratedKey {
        did,
        secret_hex: hex::encode(signing.to_bytes()),
        public_key_hex: hex::encode(pk.as_bytes()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_are_well_formed() {
        let k = generate();
        assert!(k.did.starts_with("did:sentinel:"));
        assert_eq!(k.did.len(), "did:sentinel:".len() + 64);
        assert_eq!(k.public_key_hex.len(), 64);
        assert_eq!(k.secret_hex.len(), 64);
    }

    #[test]
    fn keys_are_unique() {
        let a = generate();
        let b = generate();
        assert_ne!(a.did, b.did);
        assert_ne!(a.public_key_hex, b.public_key_hex);
    }
}

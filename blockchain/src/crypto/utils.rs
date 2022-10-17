use p256::ecdsa::{SigningKey, VerifyingKey};
use p256::elliptic_curve::rand_core::OsRng;
use serde::{Deserialize, Serialize};
use p256::pkcs8::{EncodePublicKey, LineEnding};
use p256::{PublicKey, SecretKey};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPair {
    pub private_key: String,
    pub public_key: String
}

pub fn generate_random_keypair() -> KeyPair {
    let signing_key = SigningKey::random(&mut OsRng);
    let verification_key = VerifyingKey::from(&signing_key);

    return KeyPair {
        private_key: SecretKey::from(signing_key).to_pem(LineEnding::LF).unwrap().to_string(),
        public_key: PublicKey::from(verification_key).to_public_key_pem(LineEnding::LF).unwrap()
    }
}
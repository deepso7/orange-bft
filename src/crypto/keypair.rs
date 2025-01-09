use ed25519_dalek::{ed25519::signature::SignerMut, SigningKey, VerifyingKey};
use rand::rngs::OsRng;

pub struct Keypair {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,

    // hex pub key
    publicKey: String,
}

// constructor
impl Keypair {
    pub fn new() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let public_key_bytes = verifying_key.to_bytes();
        let public_key_hex = hex::encode(&public_key_bytes);

        Self {
            signing_key,
            verifying_key,
            publicKey: public_key_hex,
        }
    }
}

// methods
impl Keypair {
    pub fn sign(&mut self, message: &[u8]) -> ed25519_dalek::Signature {
        self.signing_key.sign(message)
    }

    pub fn publicKey(&self) -> String {
        self.publicKey.to_string()
    }
}

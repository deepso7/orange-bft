use ed25519_dalek::{Signature, Verifier, VerifyingKey};

use crate::error::Result;

pub struct SignatureData {
    pub public_key: String,
    pub message: String,
    pub signature: String,
}

pub fn verify_msg(signature_data: SignatureData) -> Result<bool> {
    let public_key_bytes = hex::decode(signature_data.public_key)?;
    let signature_bytes = hex::decode(signature_data.signature)?;
    let message_bytes = signature_data.message.as_bytes();

    let verifying_key: VerifyingKey =
        VerifyingKey::from_bytes(public_key_bytes.as_slice().try_into()?)?;
    let sig = Signature::from_bytes(signature_bytes.as_slice().try_into()?);

    let valid = verifying_key.verify(&message_bytes, &sig).is_ok();

    Ok(valid)
}

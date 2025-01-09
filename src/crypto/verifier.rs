use ed25519_dalek::{Signature, Verifier, VerifyingKey};

use crate::error::Result;

pub fn verify_msg(public_key: String, message: String, signature: String) -> Result<bool> {
    let public_key_bytes = hex::decode(public_key)?;
    let signature_bytes = hex::decode(signature)?;
    let message_bytes = message.as_bytes();

    let verifying_key: VerifyingKey =
        VerifyingKey::from_bytes(public_key_bytes.as_slice().try_into()?)?;
    let sig = Signature::from_bytes(signature_bytes.as_slice().try_into()?);

    let valid = verifying_key.verify(&message_bytes, &sig).is_ok();

    Ok(valid)
}

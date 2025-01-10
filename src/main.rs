#![allow(warnings)]

use crypto::{
    keypair::Keypair,
    verifier::{verify_msg, SignatureData},
};
use ed25519_dalek::{ed25519::signature::SignerMut, SigningKey, Verifier, VerifyingKey};
use error::Result;
use rand::{rngs::OsRng, Rng};

mod consensus;
mod crypto;
mod error;
mod types;

fn main() -> Result<()> {
    println!("Hello, world!");

    let mut kp = Keypair::new();

    let message = b"Hello, Ed25519!";
    let signature: ed25519_dalek::Signature = kp.sign(message);

    let valid = verify_msg(SignatureData {
        public_key: kp.publicKey(),
        message: "Hello, Ed25519!".to_string(),
        signature: hex::encode(&signature.to_bytes()),
    })?;

    println!("is valid = {valid:?}");

    Ok(())
}

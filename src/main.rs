#![allow(warnings)]

use crypto::{keypair::Keypair, verify_msg};
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
    let signature = kp.sign(message);

    let valid = verify_msg(
        kp.publicKey(),
        "Hello, Ed25519!".to_string(),
        hex::encode(&signature.to_bytes()),
    )?;

    println!("is valid = {valid:?}");

    Ok(())
}

#![allow(warnings)]

use crypto::{
    keypair::Keypair,
    verifier::{verify_msg, SignatureData},
};
use ed25519_dalek::{ed25519::signature::SignerMut, SigningKey, Verifier, VerifyingKey};
use error::Result;
use rand::{rngs::OsRng, Rng};
use tokio::signal;
use transport::flock::Flock;

mod consensus;
mod crypto;
mod error;
mod transport;
mod types;

#[tokio::main]
async fn main() -> Result<()> {
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

    let ticket = String::from("ivupn4w25hu424vtim4prinukkemuopw3ahgh2on4clloxcamypac2hhcfu6xwjkl5nuill3el725hhepjhmep4xyie6ci5zcokpgx4haerwq5duobztulzpmfyhgmjngexhezlmmf4s42lsn5uc43tfor3w64tlfyxqiafljriihoxbaeambkaa5lwz4ayaycuacavaqabqbrqt7eb63hqd");

    let f = Flock::init(kp, ticket).await?;

    println!("initialized");

    signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");

    f.shutdown().await?;

    Ok(())
}

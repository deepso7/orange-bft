use crate::types::Signature;

struct PrivateKey;
struct PublicKey;

fn sign(data: &[u8], private_key: &PrivateKey) -> Signature {
    // Sign data with private key
    todo!()
}

fn verify(signature: &Signature, data: &[u8], public_key: &PublicKey) -> bool {
    // Verify signature
    todo!()
}

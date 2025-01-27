use serde::{Deserialize, Serialize};

use crate::{
    crypto::verifier::{verify_msg, SignatureData},
    error::Result,
};

// Vote message
#[derive(Debug, Serialize, Deserialize)]
pub struct Vote {
    pub block_hash: String,
    // pub key of node
    pub voter: String,
    pub signature: String,
}

impl Vote {
    pub fn verify(&self) -> Result<bool> {
        verify_msg(SignatureData {
            message: self.block_hash.to_string(),
            public_key: self.voter.to_string(),
            signature: self.signature.to_string(),
        })
    }
}

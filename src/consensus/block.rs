use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Digest, Sha256};

use crate::error::Result;

// TODO
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockData {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    height: u64,
    parent: String,
    block_data: BlockData,
}

impl Block {
    pub fn genesis(&self) -> Self {
        Block {
            height: 0,
            parent: "genesis".to_string(),
            block_data: BlockData {},
        }
    }

    pub fn hash(&self) -> Result<String> {
        let serialized = serde_json::to_vec(self)?;

        let mut hasher = Sha256::new();

        hasher.update(serialized);

        Ok(hex::encode(hasher.finalize()))
    }
}

// Finalize message
#[derive(Debug, Serialize, Deserialize)]
pub struct Finalize {
    pub height: u64,
    pub voter: String,
    pub signature: Vec<u8>,
}

// Node state
pub struct NodeState {
    pub current_height: u64,
    pub notarized_blocks: HashMap<u64, Block>,
    pub finalized_blocks: Vec<Block>,
}

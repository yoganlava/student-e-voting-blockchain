use std::time::{SystemTime, UNIX_EPOCH};
use ethereum_types::U256;
use serde::{Deserialize, Serialize};
use crate::primitives::transaction::Transaction;

pub type BlockIndex = u64;
// sha256 U256
pub type BlockHash = U256;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: BlockIndex,
    pub nonce: u64,
    pub timestamp: u128,
    pub previous_hash: BlockHash,
    pub hash: BlockHash,
    // 1 tx per block for simplicity
    pub transaction: Transaction,
//     pub merkle_tree_root: U256
}

impl Block {
    pub fn new(
        index: BlockIndex,
        nonce: u64,
        previous_hash: BlockHash,
    ) -> Block {
        let mut block = Block {
            index,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
            nonce,
            previous_hash,
            hash: BlockHash::default(),
            transaction: Transaction::default()
        };
        block.hash = block.calculate_hash();

        block
    }

    fn calculate_hash(&self) -> BlockHash {
        let data = serde_json::to_string(self).unwrap();
        let hash = crate::crypto::sha::hash(data.as_bytes());
        let mut hash_bytes = [0u8; 32];

        for i in 0..8 {
            let bytes = hash[i].to_be_bytes();
            hash_bytes[i] = bytes[0];
            hash_bytes[i+1] = bytes[1];
            hash_bytes[i+2] = bytes[2];
            hash_bytes[i+3] = bytes[3];
        }
        BlockHash::from(hash_bytes)
    }
}
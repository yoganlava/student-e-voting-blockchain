use std::time::{SystemTime, UNIX_EPOCH};
use ethereum_types::U256;
use crate::primitives::block::{Block, BlockHash, BlockIndex};
use crate::primitives::transaction::Transaction;

pub struct Chain {
    blocks: Vec<Block>
}

impl Chain {

    // add given block
    pub fn add_block(
        &mut self,
        block: Block
    ) {

        if block.previous_hash != self.blocks.last().unwrap().hash {
            // TODO err handling
            panic!("Invalid hash")
        }

        self.blocks.push(block);
    }

    // creates genesis block (first block in chain)
    fn create_genesis_block(&mut self) {
        self.blocks.push(Block {
            index: 0,
            nonce: 0,
            timestamp: 0,
            previous_hash: U256::zero(),
            hash: U256::zero(),
            transaction: Transaction::default()
        })
    }
}
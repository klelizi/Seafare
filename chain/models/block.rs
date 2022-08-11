use super::blockchain::Blockchain;
use chrono::prelude::*;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

// Block, a struct that represents a block in the blockchain
#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    // The index of the block
    pub index: usize,
    // The hash of the previous block
    pub previous_hash: String,
    // The hash of the block
    pub hash: String,
    // The timestamp of the block
    pub timestamp: DateTime<Utc>,
    // The data of the block
    pub data: String,
    // The difficulty of the block
    pub difficulty: usize,
    // The nonce of the block
    pub nonce: usize
// Calculate the hash of the block
    pub fn calc_hash(&self) -> String {
        let mut block_data = self.clone();
        block_data.hash = String::default();
        let serailized_block_data = serde_json::to_string(&block_data).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(serailized_block_data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
    pub fn mine (&mut self, blockchain: Blockchain) {
        loop {
            if self.hash.starts_with(&blockchain.difficulty.to_string().repeat(blockchain.difficulty)) {
                break;
            }
            self.nonce += 1;
            self.hash = self.calc_hash(); }
            else {
                self.nonce += 1;
                self.hash = self.calc_hash();
            }
        }
    }
impl Block {}

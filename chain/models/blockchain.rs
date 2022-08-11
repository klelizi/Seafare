use chrono::prelude::*;
// Internal module
use super::block::Block;

type Blocks = Vec<Block>;

// Blockchain struct that represents the blockchain
#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
    // First block in the blockchain
    pub genesis_block: Block,
    // Stores all the blocks in the blockchain
    pub chain: Blocks,
    // Stores the current difficulty of the blockchain
    pub difficulty: usize
}
impl Blockchain {}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        // Genesis Block
        let mut genesis_block = Block {
            index: 0,
            timestamp: Utc::now().timestamp_millis as u64,
            proof_of_work: u64::default(),
            previous_hash: String::default(),
            hash: String::default(),
        };
        // Create chain from genesis block
        let mut chain = vec::new();
        chain.push(genesis_block);
        // Create blockchain instance
        let blockchain = Blockchain {
            genesis_block,
            chain,
            difficulty
        };
        blockchain
    }
}

pub fn add_block(&mut self, nonce: String) {
    // Create new block
    let mut new_block = Block {
        index: self.chain.len(),
        timestamp: Utc::now().timestamp_millis as u64,
        proof_of_work: u64::default(),
        previous_hash: String::default(),
        hash: String::default(),
    };
    // Add nonce to block
    new_block.nonce = nonce.parse::<usize>().unwrap();
    // Add block to chain
    self.chain.push(new_block);
    println!("Added block: {:?}", new_block);
}


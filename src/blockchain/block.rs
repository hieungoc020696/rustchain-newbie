use std::clone::Clone;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use serde::Serialize;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Serialize)]
pub struct Block {
    pub timestamp: u64,
    pub last_hash: String,
    pub hash: String,
    pub data: String
}

impl Block {
    pub fn new(timestamp: u64, last_hash: &str, hash: &str, data: &str) -> Block {
        Block {
            timestamp,
            last_hash: last_hash.to_string(),
            hash: hash.to_string(),
            data: data.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        let s = format!("Block - \n
            Timestamp: {} \n
            Last Hash: {} \n
            Hash: {} \n
            Data: {}
        ", self.timestamp, self.last_hash, self.hash, self.data);
        s
    }

    pub fn genesis() -> Block {
        Block::new(0, "------", "f1r57-h45h", "")
    }

    pub fn mine(last_block: &Block, data: &str) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let last_hash = &last_block.hash;
        let hash = Self::hash(&timestamp, last_hash, data);
        Block::new(
            timestamp,
            last_hash,
            &hash,
            data
        )
    }

    pub fn hash(timestamp: &u64, last_hash: &str, data: &str) -> String {
        let mut hasher = Sha256::new();
        let hash_data = format!("{}{}{}", timestamp, last_hash, data);
        hasher.update(hash_data);
        let result: String = format!("{:X}", hasher.finalize());
        result
    }

    pub fn is_equal(&self, other_block: &Block) -> bool {
        self.timestamp == other_block.timestamp 
            && self.last_hash == other_block.last_hash 
            && self.hash == other_block.hash
            && self.data == other_block.data
    }

    pub fn block_hash(block: &Block) -> String {
        Block::hash(&block.timestamp, &block.last_hash, &block.data)
    }
}
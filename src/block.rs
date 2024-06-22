/**
 * Zync
 *
 * A simple blockchain implementation in Rust.
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 */
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{transaction::Transaction, utils::timestamp};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: u64,
    pub timestamp: u64,
    pub data: Vec<Transaction>,
    pub prev_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(id: u64, prev_hash: String, data: Vec<Transaction>) -> Self {
        let mut block = Block {
            id,
            timestamp: timestamp(),
            data,
            prev_hash,
            hash: String::new(),
            nonce: 0,
        };

        block.hash = block.calculate_hash();

        block
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.id.to_string());
        hasher.update(self.timestamp.to_string());
        hasher.update(serde_json::to_string(&self.data).unwrap());
        hasher.update(&self.prev_hash);
        hasher.update(self.nonce.to_string());

        format!("{:x}", hasher.finalize())
    }

    pub fn mine_block(&mut self, difficulty: u32) {
        let target = "0".repeat(difficulty as usize);

        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }

        println!("Block mined! ID: {} Hash: {}", self.id, self.hash);
    }
}

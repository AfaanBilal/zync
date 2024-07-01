/**
 * Zync
 *
 * A simple blockchain implementation in Rust.
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 */
use serde::{Deserialize, Serialize};

use crate::{block::Block, transaction::Transaction};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: u32,
    pub pending_transactions: Vec<Transaction>,
    pub mining_reward: u64,
}

impl Blockchain {
    pub fn new(difficulty: u32) -> Self {
        Blockchain {
            chain: vec![Block::new(0, String::new(), Vec::new())],
            difficulty,
            pending_transactions: Vec::new(),
            mining_reward: 100,
        }
    }

    fn last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn mine(&mut self, miner_reward_address: String) {
        let mut block = Block::new(
            self.chain.len() as u64,
            self.last_block().hash.clone(),
            self.pending_transactions.clone(),
        );

        // Mine the block (proof-of-work)
        block.mine_block(self.difficulty);

        // Add the mined block to the chain
        self.chain.push(block);

        // Reset the pending transactions and add miner reward
        let miner_reward = Transaction::new(
            String::from("[System]"),
            miner_reward_address,
            self.mining_reward,
            Some(String::from("Mining Reward")),
        );

        self.pending_transactions = vec![miner_reward];
    }

    pub fn get_balance(&self, address: &str) -> u64 {
        self.chain
            .iter()
            .flat_map(|block| &block.data)
            .fold(0, |balance, transaction| {
                if transaction.from == address {
                    balance - transaction.amount
                } else if transaction.to == address {
                    balance + transaction.amount
                } else {
                    balance
                }
            })
    }
}

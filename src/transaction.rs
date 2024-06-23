/**
 * Zync
 *
 * A simple blockchain implementation in Rust.
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 */
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};

use crate::utils::timestamp;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: u64,
    pub timestamp: u64,
    pub from: String,
    pub to: String,
    pub amount: u64,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: u64) -> Self {
        println!("[TXN] {from: >10} -> {to: >10}: {amount: >5}");

        Transaction {
            id: OsRng.next_u64(),
            timestamp: timestamp(),
            from,
            to,
            amount,
        }
    }
}

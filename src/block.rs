use crate::transaction::Transaction;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use colored::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, transactions: Vec<Transaction>, timestamp: u128) -> Self {
        let mut block = Block {
            index,
            previous_hash,
            timestamp,
            transactions,
            nonce: 0,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}{:?}{}", self.index, self.previous_hash, self.timestamp, self.transactions, self.nonce));
        format!("{:x}", hasher.finalize())
    }

    pub fn display_colorful(&self) {
        println!("{}", "Block Details:".bold().underline().blue());
        println!("{} {}", "Index:".bold().green(), self.index);
        println!("{} {}", "Previous Hash:".bold().green(), self.previous_hash);
        println!("{} {}", "Timestamp:".bold().green(), self.timestamp);
        println!("{} {:?}", "Transactions:".bold().green(), self.transactions);
        println!("{} {}", "Nonce:".bold().green(), self.nonce);
        println!("{} {}", "Hash:".bold().green(), self.hash);
    }
}

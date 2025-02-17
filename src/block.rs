use crate::transaction::Transaction;
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use colored::Colorize; // Add this import

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, transactions: Vec<Transaction>, nonce: u64) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let hash = Self::calculate_hash(index, &previous_hash, timestamp, &transactions, nonce);
        let block = Block {
            index,
            previous_hash,
            timestamp,
            transactions,
            nonce,
            hash,
        };
        block.display_colorful(); // Call the method here
        block
    }

    pub fn calculate_hash(index: u64, previous_hash: &str, timestamp: u128, transactions: &[Transaction], nonce: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(previous_hash);
        hasher.update(timestamp.to_string());
        hasher.update(format!("{:?}", transactions));
        hasher.update(nonce.to_string());
        format!("{:x}", hasher.finalize())
    }

    pub fn display_colorful(&self) {
        println!("{}", "Block Details:".bold().underline().blue());
        println!("{}: {}", "Index".bold().green(), self.index);
        println!("{}: {}", "Previous Hash".bold().green(), self.previous_hash);
        println!("{}: {}", "Timestamp".bold().green(), self.timestamp);
        println!("{}: {:?}", "Transactions".bold().green(), self.transactions);
        println!("{}: {}", "Nonce".bold().green(), self.nonce);
        println!("{}: {}", "Hash".bold().green(), self.hash);
    }
}

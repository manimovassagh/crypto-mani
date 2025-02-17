mod transaction;
mod block;
mod blockchain;

use blockchain::Blockchain;
use transaction::Transaction;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use serde_json;

fn main() {
    let blockchain_file = "blockchain_data.json";

    let mut blockchain = if Path::new(blockchain_file).exists() {
        load_blockchain(blockchain_file).unwrap_or_else(|_| Blockchain::new())
    } else {
        Blockchain::new()
    };

    let transaction1 = Transaction {
        sender: String::from("Alice"),
        receiver: String::from("Bob"),
        amount: 50,
    };

    let transaction2 = Transaction {
        sender: String::from("Bob"),
        receiver: String::from("Sahar"),
        amount: 30,
    };

    let transaction3 = Transaction {
        sender: String::from("Mani"),
        receiver: String::from("Dave"),
        amount: 20,
    };

    blockchain.add_block(vec![transaction1, transaction2, transaction3]);

    blockchain.display();

    save_blockchain(&blockchain, blockchain_file).expect("Failed to save blockchain");
}

fn load_blockchain(filename: &str) -> io::Result<Blockchain> {
    let mut file = File::open(filename)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let blockchain: Blockchain = serde_json::from_str(&data)?;
    Ok(blockchain)
}

fn save_blockchain(blockchain: &Blockchain, filename: &str) -> io::Result<()> {
    let data = serde_json::to_string(blockchain)?;
    let mut file = File::create(filename)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

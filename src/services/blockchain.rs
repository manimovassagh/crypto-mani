use crate::models::block::Block;
use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: u64,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            blocks: Vec::new(),
            difficulty: 4,
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, String::from("0"), Vec::new(), 0);
        self.blocks.push(genesis_block);
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_block = self.blocks.last().unwrap();
        let mut new_block = Block::new(
            previous_block.index + 1,
            previous_block.hash.clone(),
            transactions,
            0,
        );

        self.mine_block(&mut new_block);
        self.blocks.push(new_block);
    }

    fn mine_block(&self, block: &mut Block) {
        while !block.hash.starts_with(&"0".repeat(self.difficulty as usize)) {
            block.nonce += 1;
            block.hash = block.calculate_hash();
        }
        println!("Block mined: {}", block.hash);
    }

    pub fn display(&self) {
        for block in &self.blocks {
            println!("----------------------------------");
            block.display_colorful();
            println!("Transactions:");
            for transaction in &block.transactions {
                println!("  - {}", transaction.details());
            }
            println!("----------------------------------");
        }
    }
}

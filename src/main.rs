mod transaction;
mod block;
mod blockchain;

use blockchain::Blockchain;
use transaction::Transaction;

fn main() {
    println!("Welcome to the Mani cryptocurrency!");

    let mut blockchain = Blockchain::new();
    let transaction = Transaction::new(String::from("Alice"), String::from("Bob"), 50.0);
    println!("{}", transaction.details());
    blockchain.add_block(vec![transaction]);

    blockchain.display();
}

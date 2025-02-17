mod transaction;
mod block;
mod blockchain;

use blockchain::Blockchain;
use transaction::Transaction;

fn main() {
    let mut blockchain = Blockchain::new();

    let transaction1 = Transaction {
        sender: String::from("Alice"),
        receiver: String::from("Bob"),
        amount: 50,
    };

    let transaction2 = Transaction {
        sender: String::from("Bob"),
        receiver: String::from("Charlie"),
        amount: 30,
    };

    blockchain.add_block(vec![transaction1, transaction2]);

    blockchain.display();
}

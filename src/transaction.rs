use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub timestamp: u128,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: f64) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
        }
    }

    pub fn details(&self) -> String {
        format!(
            "Transaction from {} to {} of amount {} at {}",
            self.sender, self.receiver, self.amount, self.timestamp
        )
    }
}

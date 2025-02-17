use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}

impl Transaction {
    pub fn details(&self) -> String {
        format!("{} -> {}: {}", self.sender, self.receiver, self.amount)
    }
}

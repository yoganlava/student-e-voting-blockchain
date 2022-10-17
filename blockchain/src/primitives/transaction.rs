use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Transaction {
    from_addr: String,
    to_addr: String,
    // TODO
    signature: u64,
    amount: u64
}

impl Transaction {
    fn sign_tx(&mut self) {
        
    }
}
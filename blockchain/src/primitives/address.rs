use std::fmt;
use rand::Rng;

// Size of all addresses 32 bytes
const ADDRESS_LEN: u8 = 32;

pub struct Address([u8; 32]);

impl Address {
    // generate new random address
    pub fn new() -> Address {
        Address(rand::thread_rng().gen::<[u8; 32]>())
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}
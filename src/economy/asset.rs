use super::Monetary;
use bigdecimal::Zero;

pub struct Asset {
    balance: Monetary,
}

impl Asset {
    pub fn new() -> Asset {
        Asset {
            balance: Monetary::zero(),
        }
    }
}

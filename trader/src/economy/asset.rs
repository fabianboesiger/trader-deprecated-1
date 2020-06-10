use super::Monetary;
use bigdecimal::Zero;
use std::ops::Mul;

#[derive(Debug)]
pub struct Asset {
    symbol: String,
    balance: Monetary,
}

impl Asset {
    pub fn new(symbol: String) -> Asset {
        Asset {
            symbol,
            balance: Monetary::zero(),
        }
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    pub fn get_balance(&self) -> Monetary {
        self.balance
    }

    pub fn set_balance(&mut self, balance: Monetary) {
        debug_assert!(balance >= 0.0);
        self.balance = balance;
    }

    pub fn add_balance(&mut self, balance: Monetary) {
        debug_assert!(self.balance + balance >= 0.0);
        self.balance += balance;
    }
}

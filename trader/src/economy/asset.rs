use super::{Monetary, symbols::AssetSymbol};
use std::ops::Mul;

pub struct Asset {
    symbol: AssetSymbol,
    balance: Monetary,
}

impl Asset {
    pub fn new(symbol: String) -> Asset {
        Asset {
            symbol: symbol.into(),
            balance: 0.0,
        }
    }

    pub fn get_symbol(&self) -> &AssetSymbol {
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

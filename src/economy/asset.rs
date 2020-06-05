use bigdecimal::{Num, Zero};

#[derive(Debug)]
pub struct Asset<N>
    where
        N: Num
{
    symbol: String,
    balance: N,
}

impl<N> Asset<N>
    where
        N: Num
{
    pub fn new(symbol: String) -> Asset<N> {
        Asset {
            symbol,
            balance: N::zero(),
        }
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }
}

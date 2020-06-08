use crate::economy::Monetary;
use crate::indicators::Indicator;

#[derive(Debug)]
pub enum Action {
    Buy(Monetary, Monetary),
    Sell(Monetary, Monetary),
    Hold,
}

pub trait Trader
where
    Self::Indicators: Indicator
{
    type Indicators; 

    fn initialize(base: &str, quote: &str) -> Self;

    fn evaluate(&mut self, subscriptions: <Self::Indicators as Indicator>::Output) -> Action;
}

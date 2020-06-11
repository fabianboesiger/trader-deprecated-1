mod rsi_trader;
mod backoff;
mod stop_loss;

pub use rsi_trader::RSITrader;
pub use backoff::Backoff;
pub use stop_loss::StopLoss;

use crate::economy::Monetary;
use crate::indicators::Indicator;

#[derive(Debug)]
pub enum Order {
    Market(Action, Monetary),
    Limit(Action, Monetary, Monetary),
}

#[derive(Debug)]
pub enum Action {
    Buy,
    Sell
}

pub trait Trader
where
    Self::Indicators: Indicator,
{
    type Indicators;

    fn initialize(base: &str, quote: &str) -> Self;

    fn evaluate(&mut self, output: <Self::Indicators as Indicator>::Output) -> Option<Order>;
}

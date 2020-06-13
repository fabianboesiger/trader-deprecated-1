mod rsi_trader;
mod macd_trader;
mod backoff;
mod stop_loss;
mod full_stop;
mod gobble_bad_longterm;
mod and;
mod safe_and;

pub use rsi_trader::RSITrader;
pub use macd_trader::MACDTrader;
pub use backoff::Backoff;
pub use stop_loss::StopLoss;
pub use full_stop::FullStop;
pub use gobble_bad_longterm::GobbleBadLongterm;
pub use and::And;
pub use safe_and::SafeAnd;

use crate::economy::Monetary;
use crate::indicators::Indicator;

#[derive(Debug, Clone)]
pub enum Order {
    Market(Action, Monetary),
    Limit(Action, Monetary, Monetary),
}

#[derive(Debug, Clone)]
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

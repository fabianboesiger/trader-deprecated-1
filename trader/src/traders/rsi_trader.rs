use super::{Action, Trader};
use crate::economy::Monetary;
use crate::indicators::{Indicator, Value, RSI, SMA};

pub struct RSITrader<const PERIOD: usize, const BUY: Monetary, const SELL: Monetary>;

impl<const PERIOD: usize, const BUY: Monetary, const SELL: Monetary> Trader
    for RSITrader<PERIOD, BUY, SELL>
{
    type Indicators = (Value, RSI<SMA<PERIOD>>);

    fn initialize(base: &str, quote: &str) -> RSITrader<PERIOD, BUY, SELL> {
        RSITrader {}
    }

    fn evaluate(&mut self, (value, rsi): <Self::Indicators as Indicator>::Output) -> Action {
        if let Some(rsi) = rsi {
            if rsi < BUY {
                return Action::Buy(0.1, value);
            } else if rsi > SELL {
                return Action::Sell(0.1, value);
            }
        }
        Action::Hold
    }
}

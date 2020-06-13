use super::{Order, Action, Trader};
use crate::economy::Monetary;
use crate::indicators::{Indicator, Value, StretchedRSI, EMA};

pub struct RSITrader<const PERIOD: usize, const SCALE: usize, const BUY: Monetary, const SELL: Monetary, const FRACTION: Monetary> {
    previous_rsi: Monetary,
}

impl<const PERIOD: usize, const SCALE: usize, const BUY: Monetary, const SELL: Monetary, const FRACTION: Monetary> Trader
    for RSITrader<PERIOD, SCALE, BUY, SELL, FRACTION>
{
    type Indicators = (Value, StretchedRSI<EMA<PERIOD>, EMA<SCALE>>);

    fn initialize(base: &str, quote: &str) -> RSITrader<PERIOD, SCALE, BUY, SELL, FRACTION> {
        RSITrader {
            previous_rsi: 50.0
        }
    }

    fn evaluate(&mut self, (value, rsi): <Self::Indicators as Indicator>::Output) -> Option<Order> {
        if let Some(rsi) = rsi {
            let action = if self.previous_rsi < BUY && rsi >= BUY {
                Some(Order::Limit(Action::Buy, FRACTION, value))
            } else if self.previous_rsi > SELL && rsi <= SELL {
                Some(Order::Limit(Action::Sell, FRACTION, value))
            } else {
                None
            };

            self.previous_rsi = rsi;

            action
        } else {
            None
        }
    }
}

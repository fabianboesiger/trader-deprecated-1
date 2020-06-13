use super::{Order, Action, Trader};
use crate::economy::Monetary;
use crate::indicators::{Indicator, Value, StretchedRSI, SMA, MACDHistogram};

pub struct MACDTrader<const FRACTION: Monetary> {
    previous_macdh: Monetary,
}

impl<const FRACTION: Monetary> Trader
    for MACDTrader<FRACTION>
{
    type Indicators = (Value, MACDHistogram<720, 1560, 540>);

    fn initialize(base: &str, quote: &str) -> MACDTrader<FRACTION> {
        MACDTrader {
            previous_macdh: 0.0
        }
    }

    fn evaluate(&mut self, (value, macd): <Self::Indicators as Indicator>::Output) -> Option<Order> {
        if let Some((macd, macdh)) = macd {
            let action = if
                self.previous_macdh < 0.0 &&
                macdh >= 0.0 &&
                macd < 0.0
            {
                Some(Order::Limit(Action::Buy, FRACTION, value))
            } else if
                self.previous_macdh > 0.0 &&
                macdh <= 0.0 &&
                macd > 0.0
            {
                Some(Order::Limit(Action::Sell, FRACTION, value))
            } else {
                None
            };

            self.previous_macdh = macd;

            action
        } else {
            None
        }
    }
}

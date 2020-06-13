use super::{Order, Action, Trader};
use crate::economy::Monetary;
use crate::indicators::{Indicator, Value, StretchedRSI, SMA, MACDHistogram, MACD};

pub struct MACDTrader<const FRACTION: Monetary> {
    previous_macdh: Monetary,
}

impl<const FRACTION: Monetary> Trader
    for MACDTrader<FRACTION>
{
    type Indicators = (Value, MACDHistogram<720, 1560, 540>, MACDHistogram<5760, 12480, 4320>);

    fn initialize(base: &str, quote: &str) -> MACDTrader<FRACTION> {
        MACDTrader {
            previous_macdh: 0.0
        }
    }

    fn evaluate(&mut self, (value, macd, lmacd): <Self::Indicators as Indicator>::Output) -> Option<Order> {
        if let (Some((macd, macdh)), Some((lmacd, lmacdh))) = (macd, lmacd) {
            let action = if
                self.previous_macdh < 0.0 &&
                macdh >= 0.0 &&
                macd < 0.0 &&
                lmacdh >= 0.0 &&
                lmacd < 0.0
            {
                Some(Order::Limit(Action::Buy, FRACTION, value))
            } else if
                self.previous_macdh > 0.0 &&
                macdh <= 0.0 &&
                macd > 0.0 &&
                lmacdh <= 0.0 &&
                lmacd > 0.0
            {
                Some(Order::Limit(Action::Sell, FRACTION, value))
            } else {
                None
            };

            self.previous_macdh = macdh;

            action
        } else {
            None
        }
    }
}

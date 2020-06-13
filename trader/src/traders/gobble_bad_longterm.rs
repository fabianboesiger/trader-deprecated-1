use super::{Trader, Order, Action};
use crate::indicators::{MACDHistogram, Indicator};
use crate::economy::Monetary;

enum Safe {
    Base,
    Quote,
    None
}

pub struct GobbleBadLongterm<T, const SAFE: &'static str>
    where
        T: Trader
{
    trader: T,
    safe: Safe,
}

impl<T, const SAFE: &'static str> Trader for GobbleBadLongterm<T, SAFE>
    where
        T: Trader
{ 
    type Indicators = (MACDHistogram<21600, 46800, 16200>, T::Indicators);

    fn initialize(base: &str, quote: &str) -> GobbleBadLongterm<T, SAFE> {
        GobbleBadLongterm {
            trader: T::initialize(base, quote),
            safe: if base == SAFE {
                    Safe::Base
                } else
                if quote == SAFE {
                    Safe::Quote
                } else {
                    Safe::None
                }
        }
    }

    fn evaluate(&mut self, (macd, output): <Self::Indicators as Indicator>::Output) -> Option<Order> {
        if let (Some(order), Some((macd, macdh))) = (self.trader.evaluate(output), macd) {
            match (&self.safe, order) {
                (Safe::Quote, Order::Limit(Action::Buy, quantity, value)) => {
                    if macdh > 0.0 {
                        Some(Order::Limit(Action::Buy, quantity, value))
                    } else {
                        None
                    }
                },
                (Safe::Base, Order::Limit(Action::Sell, quantity, value)) => {
                    if macdh < 0.0 {
                        Some(Order::Limit(Action::Sell, quantity, value))
                    } else {
                        None
                    }
                },
                (_, order) => {
                    Some(order)
                }
            }
        } else {
            None
        }
        
    }
}
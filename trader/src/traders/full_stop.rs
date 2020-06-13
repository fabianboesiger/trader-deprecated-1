use super::{Trader, Order, Action};
use crate::indicators::{Value, Indicator};
use crate::economy::Monetary;

enum Safe {
    Base,
    Quote,
    None
}

pub struct FullStop<T, const SAFE: &'static str>
    where
        T: Trader
{
    trader: T,
    safe: Safe,
}

impl<T, const SAFE: &'static str> Trader for FullStop<T, SAFE>
    where
        T: Trader
{ 
    type Indicators = T::Indicators;

    fn initialize(base: &str, quote: &str) -> FullStop<T, SAFE> {
        FullStop {
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

    fn evaluate(&mut self, output: <Self::Indicators as Indicator>::Output) -> Option<Order> {
        if let Some(order) = self.trader.evaluate(output) {
            Some(match (&self.safe, order) {
                (Safe::Base, Order::Limit(Action::Buy, quantity, value)) => {
                    Order::Limit(Action::Buy, 1.0, value)
                },
                (Safe::Quote, Order::Limit(Action::Sell, quantity, value)) => {
                    Order::Limit(Action::Sell, 1.0, value)
                },
                (_, order) => {
                    order
                }
            })
        } else {
            None
        }
        
    }
}
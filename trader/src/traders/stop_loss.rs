use super::{Trader, Order, Action};
use crate::indicators::{Value, Indicator};
use crate::economy::Monetary;

enum Safe {
    Base,
    Quote,
    None
}

pub struct StopLoss<T, const SAFE: &'static str, const STOP: Monetary>
    where
        T: Trader
{
    trader: T,
    safe: Safe,
    stop: Option<Monetary>,
}

impl<T, const SAFE: &'static str, const STOP: Monetary> Trader for StopLoss<T, SAFE, STOP>
    where
        T: Trader
{ 
    type Indicators = (Value, T::Indicators);

    fn initialize(base: &str, quote: &str) -> StopLoss<T, SAFE, STOP> {
        StopLoss {
            trader: T::initialize(base, quote),
            safe: if base == SAFE {
                    Safe::Base
                } else
                if quote == SAFE {
                    Safe::Quote
                } else {
                    Safe::None
                },
            stop: None,
        }
    }

    fn evaluate(&mut self, (value, output): <Self::Indicators as Indicator>::Output) -> Option<Order> {
        if let Some(stop) = self.stop {
            match self.safe {
                Safe::Base => {
                    if value >= stop {
                        self.stop = None;
                        return Some(Order::Market(Action::Buy, 1.0));
                    }
                },
                Safe::Quote => {
                    if value <= stop {
                        self.stop = None;
                        return Some(Order::Market(Action::Sell, 1.0));
                    }
                }
                Safe::None => {}
            }
        }

        let order = self.trader.evaluate(output);
        if let Some(order) = &order {
            match (&self.safe, order) {
                (Safe::Quote, Order::Limit(Action::Buy, _, value)) => {
                    self.stop = Some(if let Some(stop) = self.stop {
                        stop.min(*value * STOP)
                    } else {
                        *value * STOP
                    });
                },
                (Safe::Base, Order::Limit(Action::Sell, _, value)) => {
                    self.stop = Some(if let Some(stop) = self.stop {
                        stop.max(*value / STOP)
                    } else {
                        *value / STOP
                    });
                },
                _ => {}
            }
        }
        order
    }
}
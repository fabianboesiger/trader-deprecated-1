use super::{Trader, Order, Action};
use crate::indicators::{Value, Indicator};
use crate::economy::Monetary;

enum Safe {
    Base,
    Quote,
    None
}

pub struct SafeAnd<T1, T2, const SAFE: &'static str>
    where
        T1: Trader,
        T2: Trader
{
    trader1: T1,
    trader2: T2,
    safe: Safe,
}

impl<T1, T2, const SAFE: &'static str> Trader for SafeAnd<T1, T2, SAFE>
    where
        T1: Trader,
        T2: Trader
{ 
    type Indicators = (T1::Indicators, T2::Indicators);

    fn initialize(base: &str, quote: &str) -> SafeAnd<T1, T2, SAFE> {
        SafeAnd {
            trader1: T1::initialize(base, quote),
            trader2: T2::initialize(base, quote),
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

    fn evaluate(&mut self, (output1, output2): <Self::Indicators as Indicator>::Output) -> Option<Order> {
        match self.safe {
            Safe::Base => {
                match (self.trader1.evaluate(output1), self.trader2.evaluate(output2)) {
                    (Some(Order::Limit(Action::Buy, quantity1, value1)), Some(Order::Limit(Action::Buy, quantity2, value2))) => {
                        Some(Order::Limit(Action::Buy, (quantity1 + quantity2) / 2.0, (value1 + value2) / 2.0))
                    },
                    (Some(Order::Limit(Action::Buy, quantity1, value1)), _) => {
                        Some(Order::Limit(Action::Buy, quantity1, value1))
                    },
                    (_, Some(Order::Limit(Action::Buy, quantity2, value2))) => {
                        Some(Order::Limit(Action::Buy, quantity2, value2))
                    },
                    (Some(Order::Limit(Action::Sell, quantity1, value1)), Some(Order::Limit(Action::Sell, quantity2, value2))) => {
                        Some(Order::Limit(Action::Sell, (quantity1 + quantity2) / 2.0, (value1 + value2) / 2.0))
                    },
                    (_, _) => {
                        None
                    }
                }
            },
            Safe::Quote => {
                match (self.trader1.evaluate(output1), self.trader2.evaluate(output2)) {
                    (Some(Order::Limit(Action::Buy, quantity1, value1)), Some(Order::Limit(Action::Buy, quantity2, value2))) => {
                        Some(Order::Limit(Action::Buy, (quantity1 + quantity2) / 2.0, (value1 + value2) / 2.0))
                    },
                    (Some(Order::Limit(Action::Sell, quantity1, value1)), Some(Order::Limit(Action::Sell, quantity2, value2))) => {
                        Some(Order::Limit(Action::Sell, (quantity1 + quantity2) / 2.0, (value1 + value2) / 2.0))
                    },
                    (Some(Order::Limit(Action::Sell, quantity1, value1)), _) => {
                        Some(Order::Limit(Action::Sell, quantity1, value1))
                    },
                    (_, Some(Order::Limit(Action::Sell, quantity2, value2))) => {
                        Some(Order::Limit(Action::Sell, quantity2, value2))
                    },
                    (_, _) => {
                        None
                    }
                }
            },
            Safe::None => {
                match (self.trader1.evaluate(output1), self.trader2.evaluate(output2)) {
                    (Some(Order::Limit(Action::Buy, quantity1, value1)), Some(Order::Limit(Action::Buy, quantity2, value2))) => {
                        Some(Order::Limit(Action::Buy, (quantity1 + quantity2) / 2.0, (value1 + value2) / 2.0))
                    },
                    (Some(Order::Limit(Action::Sell, quantity1, value1)), Some(Order::Limit(Action::Sell, quantity2, value2))) => {
                        Some(Order::Limit(Action::Sell, (quantity1 + quantity2) / 2.0, (value1 + value2) / 2.0))
                    },
                    (_, _) => {
                        None
                    }
                }
            }
        }
    }
}
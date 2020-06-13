use super::{Trader, Order, Action};
use crate::indicators::{Value, Indicator};
use crate::economy::Monetary;

pub struct And<T1, T2>
    where
        T1: Trader,
        T2: Trader
{
    trader1: T1,
    trader2: T2
}

impl<T1, T2> Trader for And<T1, T2>
    where
        T1: Trader,
        T2: Trader
{ 
    type Indicators = (T1::Indicators, T2::Indicators);

    fn initialize(base: &str, quote: &str) -> And<T1, T2> {
        And {
            trader1: T1::initialize(base, quote),
            trader2: T2::initialize(base, quote),
        }
    }

    fn evaluate(&mut self, (output1, output2): <Self::Indicators as Indicator>::Output) -> Option<Order> {
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
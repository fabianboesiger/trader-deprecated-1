use super::{Trader, Order, Action};
use crate::indicators::Indicator;

pub struct Backoff<T, const BACKOFF: usize>
    where
        T: Trader
{
    trader: T,
    backoff: usize,
}

impl<T, const BACKOFF: usize> Trader for Backoff<T, BACKOFF>
    where
        T: Trader
{ 
    type Indicators = T::Indicators;

    fn initialize(base: &str, quote: &str) -> Backoff<T, BACKOFF> {
        Backoff {
            trader: T::initialize(base, quote),
            backoff: 0,
        }
    }

    fn evaluate(&mut self, output: <Self::Indicators as Indicator>::Output) -> Option<Order> {
        if self.backoff == 0 {
            let order = self.trader.evaluate(output);
            match order {
                Some(Order::Limit(Action::Buy, _, _)) | Some(Order::Limit(Action::Sell, _, _)) => {
                    self.backoff = BACKOFF;
                },
                Some(_) | None => {}
            }
            order
        } else {
            self.backoff -= 1;
            None
        }
    }
}
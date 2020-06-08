use super::{Indicator, EMA};
use crate::economy::Monetary;

pub struct MACD<const SHORT: u64, const LONG: u64> {
    macd: Monetary,
    ema_short: EMA<SHORT>,
    ema_long: EMA<LONG>,
}

impl<const SHORT: u64, const LONG: u64> Indicator for MACD<SHORT, LONG> {
    type Output = Option<Monetary>;

    fn initialize(value: Monetary) -> Self {
        MACD {
            macd: 0.0,
            ema_short: EMA::initialize(value),
            ema_long: EMA::initialize(value),
        }
    }

    fn evaluate(&mut self, value: Monetary) -> Self::Output {
        if let (Some(short), Some(long)) =
            (self.ema_short.evaluate(value), self.ema_long.evaluate(value))
        {
            self.macd = short - long;
            Some(self.macd)
        } else {
            None
        }
    }
}

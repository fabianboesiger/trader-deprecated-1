use super::{Indicator, EMA};
use crate::economy::Monetary;

pub struct MACD<const SHORT: usize, const LONG: usize> {
    ema_short: EMA<SHORT>,
    ema_long: EMA<LONG>,
}

impl<const SHORT: usize, const LONG: usize> Indicator for MACD<SHORT, LONG> {
    type Output = Option<Monetary>;

    fn initialize(value: Monetary) -> Self {
        MACD {
            ema_short: EMA::initialize(value),
            ema_long: EMA::initialize(value),
        }
    }

    fn evaluate(&mut self, value: Monetary) -> Self::Output {
        if let (Some(short), Some(long)) = (
            self.ema_short.evaluate(value),
            self.ema_long.evaluate(value),
        ) {
            Some(short - long)
        } else {
            None
        }
    }
}

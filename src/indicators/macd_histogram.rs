use super::{Indicator, EMA};
use crate::economy::Monetary;
use std::time::Duration;
use bigdecimal::Zero;

pub struct MACD<const SHORT: Duration, const LONG: Duration> {
    macd: Monetary,
    ema_short: EMA<SHORT>,
    ema_long: EMA<LONG>,
}

impl<const SHORT: Duration, const LONG: Duration> Indicator for MACD<SHORT, LONG> {
    type Output<'a> = &'a Monetary;

    fn initialize(value: &Monetary) -> Self {
        MACD {
            macd: Monetary::zero(),
            ema_short: EMA::initialize(value),
            ema_long: EMA::initialize(value),
        }
    }

    fn evaluate<'a>(&'a mut self, value: &'a Monetary) -> Self::Output<'a> {
        self.macd = self.ema_short.evaluate(value) - self.ema_long.evaluate(value);
        &self.macd
    }
}

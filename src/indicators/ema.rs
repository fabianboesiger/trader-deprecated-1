use super::Indicator;
use crate::economy::Monetary;
use std::time::Duration;
use bigdecimal::One;

pub struct EMA<const PERIOD: Duration> {
    ema: Monetary
}

impl<const PERIOD: Duration> Indicator for EMA<PERIOD> {
    type Output<'a> = &'a Monetary;

    fn initialize(value: &Monetary) -> Self {
        EMA {
            ema: value.clone()
        }
    }

    fn evaluate<'a>(&'a mut self, value: &'a Monetary) -> Self::Output<'a> {
        let multiplier: &Monetary = &(2.0 / 1.0 + PERIOD.as_secs() as f64).into();
        self.ema *= Monetary::one() - multiplier;
        self.ema += value * multiplier;
        &self.ema
    }
}

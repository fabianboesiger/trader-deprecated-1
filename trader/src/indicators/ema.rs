use super::Indicator;
use crate::economy::Monetary;

pub struct EMA<const PERIOD: u64> {
    ema: Monetary,
    count: u64,
}

impl<const PERIOD: u64> Indicator for EMA<PERIOD> {
    type Output = Option<Monetary>;

    fn initialize(value: Monetary) -> Self {
        EMA {
            ema: value.clone(),
            count: 0,
        }
    }

    fn evaluate(&mut self, value: Monetary) -> Self::Output {
        let multiplier = 2.0 / (1.0 + PERIOD as f64);
        self.ema *= 1.0 - multiplier;
        self.ema += value * multiplier;

        if self.count > PERIOD {
            Some(self.ema)
        } else {
            self.count += 1;
            None
        }
    }
}

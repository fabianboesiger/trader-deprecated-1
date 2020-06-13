use super::{Indicator, MovingAverage};
use crate::economy::Monetary;

pub struct SMMA<const PERIOD: usize> {
    smma: Monetary,
    count: usize,
}

impl<const PERIOD: usize> Indicator for SMMA<PERIOD> {
    type Output = Option<Monetary>;

    fn initialize(value: Monetary) -> Self {
        SMMA {
            smma: value,
            count: 0,
        }
    }

    fn evaluate(&mut self, value: Monetary) -> Self::Output {
        self.count += 1;

        let alpha = 1.0 / PERIOD as f64;
        self.smma *= 1.0 - alpha;
        self.smma += value * alpha;

        if self.count >= PERIOD {
            Some(self.smma)
        } else {
            None
        }
    }
}

impl<const PERIOD: usize> MovingAverage for SMMA<PERIOD> {}

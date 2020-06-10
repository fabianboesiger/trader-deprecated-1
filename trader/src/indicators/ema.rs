use super::{Indicator, MovingAverage};
use crate::economy::Monetary;

pub struct EMA<const PERIOD: usize> {
    ema: Monetary,
    count: usize,
}

impl<const PERIOD: usize> Indicator for EMA<PERIOD> {
    type Output = Option<Monetary>;

    fn initialize(value: Monetary) -> Self {
        EMA {
            ema: value,
            count: 0,
        }
    }

    fn evaluate(&mut self, value: Monetary) -> Self::Output {
        self.count += 1;

        let multiplier = 2.0 / (1.0 + PERIOD as f64);
        self.ema *= 1.0 - multiplier;
        self.ema += value * multiplier;

        if self.count >= PERIOD {
            Some(self.ema)
        } else {
            None
        }
    }
}

impl<const PERIOD: usize> MovingAverage for EMA<PERIOD> {}

#[tokio::test]
async fn test_ema() {
    let mut ema = EMA::<9>::initialize(0.0);
    for i in 0..8 {
        assert_eq!(ema.evaluate(i as f64), None);
    }
    let result = ema.evaluate(8.0).unwrap();
    assert!(result > 4.6 && result < 4.7);
}

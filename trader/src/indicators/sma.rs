use super::{Indicator, MovingAverage};
use crate::economy::Monetary;

pub struct SMA<const PERIOD: usize> {
    sma: Monetary,
    count: usize,
    values: [Monetary; PERIOD],
}

impl<const PERIOD: usize> Indicator for SMA<PERIOD> {
    type Output = Option<Monetary>;

    fn initialize(value: Monetary) -> Self {
        SMA {
            sma: value.clone(),
            count: 0,
            values: [value; PERIOD],
        }
    }

    fn evaluate(&mut self, value: Monetary) -> Self::Output {
        self.count += 1;

        self.sma = value;
        for i in 1..self.values.len() {
            self.values[i - 1] = self.values[i];
            self.sma += self.values[i];
        }
        self.values[self.values.len() - 1] = value;
        self.sma /= PERIOD as Monetary;

        if self.count >= PERIOD {
            Some(self.sma)
        } else {
            None
        }
    }
}

impl<const PERIOD: usize> MovingAverage for SMA<PERIOD> {}

#[tokio::test]
async fn test_sma() {
    let mut sma = SMA::<9>::initialize(0.0);
    for i in 0..8 {
        assert_eq!(sma.evaluate(i as f64), None);
    }
    assert_eq!(sma.evaluate(8.0), Some(4.0));
}

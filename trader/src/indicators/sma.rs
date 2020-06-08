use super::Indicator;
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
        self.sma = value;
        for i in 1..self.values.len() {
            self.values[i - 1] = self.values[i];
            self.sma += self.values[i];
        }
        self.values[self.values.len() - 1] = value;
        self.sma /= PERIOD as Monetary;
        

        if self.count > PERIOD {
            Some(self.sma)
        } else {
            self.count += 1;
            None
        }
    }
}

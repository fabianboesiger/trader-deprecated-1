use super::{Indicator, MovingAverage, SMA};
use crate::economy::Monetary;

pub struct StretchedRSI<MA, STRETCH>
where
    MA: MovingAverage<Output = Option<Monetary>>,
    STRETCH: MovingAverage<Output = Option<Monetary>>,
{
    up: MA,
    down: MA,
    previous_value: Monetary,
    change: STRETCH
}

impl<MA, STRETCH> Indicator for StretchedRSI<MA, STRETCH>
where
    MA: MovingAverage<Output = Option<Monetary>>,
    STRETCH: MovingAverage<Output = Option<Monetary>>,
{
    type Output = Option<Monetary>;

    fn initialize(value: Monetary) -> Self {
        StretchedRSI {
            up: MA::initialize(value),
            down: MA::initialize(value),
            previous_value: value,
            change: STRETCH::initialize(0.0)
        }
    }

    fn evaluate(&mut self, value: Monetary) -> Self::Output {
        if let Some(change) = self.change.evaluate(value - self.previous_value) {
            self.previous_value = value;
            if let (Some(up), Some(down)) = (
                self.up.evaluate(if change > 0.0 { change } else { 0.0 }),
                self.down.evaluate(if change < 0.0 { -change } else { 0.0 }),
            ) {
                let rs = up / down;
                Some(100.0 - 100.0 / (1.0 + rs))
            } else {
                None
            }
        } else {
            None
        }
    }
}

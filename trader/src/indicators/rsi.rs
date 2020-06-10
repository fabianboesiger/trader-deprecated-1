use super::{Indicator, MovingAverage};
use crate::economy::Monetary;

pub struct RSI<MA>
where
    MA: MovingAverage<Output = Option<Monetary>>,
{
    up: MA,
    down: MA,
    previous_value: Monetary,
}

impl<MA> Indicator for RSI<MA>
where
    MA: MovingAverage<Output = Option<Monetary>>,
{
    type Output = Option<Monetary>;

    fn initialize(value: Monetary) -> Self {
        RSI {
            up: MA::initialize(value),
            down: MA::initialize(value),
            previous_value: value,
        }
    }

    fn evaluate(&mut self, value: Monetary) -> Self::Output {
        let change = value - self.previous_value;
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
    }
}

use super::{Indicator, EMA, MACD};
use crate::economy::Monetary;

pub struct MACDHistogram<const SHORT: usize, const LONG: usize, const PERIOD: usize> {
    histogram: Monetary,
    macd: MACD<SHORT, LONG>,
    signal: EMA<PERIOD>,
}

impl<const SHORT: usize, const LONG: usize, const PERIOD: usize> Indicator
    for MACDHistogram<SHORT, LONG, PERIOD>
{
    type Output = Option<(Monetary, Monetary)>;

    fn initialize(value: Monetary) -> Self {
        MACDHistogram {
            histogram: 0.0,
            macd: MACD::initialize(value),
            signal: EMA::initialize(0.0),
        }
    }

    fn evaluate(&mut self, value: Monetary) -> Self::Output {
        if let Some(macd) = self.macd.evaluate(value) {
            if let Some(signal) = self.signal.evaluate(macd) {
                self.histogram = macd - signal;
                return Some((macd, self.histogram));
            }
        }
        None
    }
}

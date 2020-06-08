use super::{Indicator, EMA, MACD};
use crate::economy::Monetary;

pub struct MACDHistogram<const SHORT: u64, const LONG: u64, const PERIOD: u64> {
    histogram: Monetary,
    macd: MACD<SHORT, LONG>,
    signal: EMA<PERIOD>,
}

impl<const SHORT: u64, const LONG: u64, const PERIOD: u64> Indicator for MACDHistogram<SHORT, LONG, PERIOD> {
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

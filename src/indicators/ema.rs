use super::Indicator;
use std::time::Duration;
use bigdecimal::{Num, One};

pub struct EMA<N, const PERIOD: Duration>
    where
        N: Num
{
    ema: N,
}

impl<N, const PERIOD: Duration> Indicator<N> for EMA<N, PERIOD>
    where
        N: Num + 'static + From<f32>
{
    type Output<'a> = &'a N;

    fn initialize(value: &N) -> Self {
        EMA {
            ema: *value
        }
    }

    fn evaluate<'a>(&'a mut self, value: &'a N) -> Self::Output<'a> {
        let multiplier: N = N::from(2.0 / (1.0 + PERIOD.as_secs() as f32));
        self.ema = *value * multiplier +  self.ema * (N::one() - multiplier);
        &self.ema
    }
}

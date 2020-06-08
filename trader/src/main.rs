#![allow(incomplete_features)]
#![feature(const_generics)]

mod economy;
mod environments;
mod indicators;
mod traders;

use chrono::{DateTime, NaiveDateTime, Utc};
use economy::{Economy, Monetary};
use environments::{Historical, Simulated};
use traders::{Action, Trader};
use indicators::{Indicator, Value, MACDHistogram};
use bigdecimal::Zero;



struct MyTrader {
    previous_macd_histogram: Monetary,
    investment_fraction: Monetary,
}

impl Trader for MyTrader {
    type Indicators = (Value, MACDHistogram<12, 26, 9>, MACDHistogram<72, 156, 54>, MACDHistogram<432, 936, 324>);

    fn initialize(base: &str, quote: &str) -> MyTrader {
        MyTrader {
            previous_macd_histogram: Monetary::zero(),
            investment_fraction: 0.1
        }
    }

    fn evaluate(&mut self, (value, macd1, macd2, macd3): <Self::Indicators as Indicator>::Output) -> Action {
        if let (Some((macd1, macd1h)), Some((macd2, macd2h)), Some((macd3, macd3h))) = (macd1, macd2, macd3) {
            let output =
                if self.previous_macd_histogram < 0.0 &&
                    macd1h > 0.0 &&
                    macd1 < 0.0 &&
                    macd2h > 0.0 &&
                    macd2 < 0.0 &&
                    macd3h > 0.0 &&
                    macd3 < 0.0
                {
                    Action::Buy(self.investment_fraction, value.clone())
                } else
                if self.previous_macd_histogram > 0.0 &&
                    macd1h < 0.0 &&
                    macd1 > 0.0 &&
                    macd2h > 0.0 &&
                    macd2 < 0.0
                {
                    Action::Sell(self.investment_fraction, value.clone())
                } else {
                    Action::Hold
                };

            self.previous_macd_histogram = macd1h.clone();

            output
        } else {
            Action::Hold
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let environment = /*Historical::new(DateTime::from_utc(
        NaiveDateTime::from_timestamp(26518062 * 60, 0),
        Utc,
    ))
    .await*/ Simulated::new().await;

    let mut economy = Economy::<_, MyTrader>::new(environment);
    economy.run().await?;

    Ok(())
}

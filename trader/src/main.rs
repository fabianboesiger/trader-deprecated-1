#![allow(incomplete_features)]
#![feature(const_generics)]

mod economy;
mod environments;
mod indicators;
mod traders;

use bigdecimal::Zero;
use chrono::{DateTime, NaiveDateTime, Utc};
use economy::{Economy, Monetary};
use environments::{Historical, Simulated};
use indicators::{Indicator, MACDHistogram, Value};
use traders::{Action, RSITrader, Trader};

struct MyTrader {
    macd1hp: Monetary,
    investment_fraction: Monetary,
}

impl Trader for MyTrader {
    type Indicators = (
        Value,
        MACDHistogram<12, 26, 9>,
        MACDHistogram<72, 156, 54>,
        MACDHistogram<432, 936, 324>,
    );

    fn initialize(base: &str, quote: &str) -> MyTrader {
        MyTrader {
            macd1hp: Monetary::zero(),
            investment_fraction: 0.1,
        }
    }

    fn evaluate(
        &mut self,
        (value, macd1, macd2, macd3): <Self::Indicators as Indicator>::Output,
    ) -> Action {
        if let (Some((macd1, macd1h)), Some((macd2, macd2h)), Some((macd3, macd3h))) =
            (macd1, macd2, macd3)
        {
            let distance = (self.macd1hp - macd1h).abs() / value;

            let output = if self.macd1hp < 0.0
                && distance > 0.00001
                && macd1h > 0.0
                && macd1 < 0.0
                && macd2h > 0.0
                && macd2 < 0.0
            /* &&
            macd3h > 0.0 &&
            macd3 < 0.0*/
            {
                Action::Buy(self.investment_fraction, value)
            } else if self.macd1hp > 0.0
                && distance > 0.00001
                && macd1h < 0.0
                && macd1 > 0.0
                && macd2h > 0.0
                && macd2 < 0.0
            {
                Action::Sell(self.investment_fraction, value)
            } else {
                Action::Hold
            };

            self.macd1hp = macd1h;

            output
        } else {
            Action::Hold
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let environment = Historical::new(DateTime::from_utc(
        NaiveDateTime::from_timestamp(26518062 * 60, 0),
        Utc,
    ))
    .await /*Simulated::new().await*/;

    let mut economy = Economy::<_, RSITrader<14, 30.0, 70.0>>::new(environment);
    economy.run().await?;

    Ok(())
}

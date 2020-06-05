#![allow(incomplete_features)]
#![feature(const_generics)]
#![feature(generic_associated_types)]

mod economy;
mod environments;
mod indicators;
mod traders;

use chrono::{DateTime, NaiveDateTime, Utc};
use economy::Economy;
use environments::Historical;
use traders::{Action, Trader};
use indicators::{Value, EMA};
use std::time::Duration;

const D12: Duration = Duration::from_secs(12);

struct MyTrader;

impl Trader for MyTrader {
    type Subscriptions = (Value, EMA<D12>);

    fn evaluate((_value, _ema): Self::Subscriptions) -> Action {
        Action::Hold
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let environment = Historical::new(DateTime::from_utc(
        NaiveDateTime::from_timestamp(26518062 * 60, 0),
        Utc,
    ))
    .await;

    let mut economy = Economy::<Historical, MyTrader>::new(environment);
    economy.run().await?;

    Ok(())
}

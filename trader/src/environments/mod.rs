mod historical;
mod simulated;

pub use historical::Historical;
pub use simulated::Simulated;

use crate::economy::{Monetary, Market};
use crate::traders::Action;
use async_trait::async_trait;
use std::time::Duration;
use std::fmt::Debug;
use binance_async::model::{Symbol, Order};

#[derive(Debug)]
pub enum Event {
    SetMarketValue(String, Monetary),
    SetAssetBalance(String, Monetary),
    Evaluate,
}

pub type MarketData = Symbol;
pub type OrderData = Order;

#[async_trait]
pub trait Environment {
    async fn initialize(&mut self) -> Result<Vec<MarketData>, ()>;
    async fn poll(&mut self) -> Event;
    async fn order(&mut self, order: OrderData) -> Result<(), ()>;
}

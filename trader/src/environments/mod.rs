mod historical;
mod simulated;

pub use historical::Historical;
pub use simulated::Simulated;

use crate::economy::{Market, Monetary};
use crate::traders::Action;
use async_trait::async_trait;
use binance_async::model::{Order, Symbol};
use std::fmt::Debug;
use std::time::Duration;

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

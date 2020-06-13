//mod historical;
mod simulated;

//pub use historical::Historical;
pub use simulated::Simulated;

use crate::economy::{Market, Monetary, AssetSymbol};
use crate::traders::{Order, Action};
use async_trait::async_trait;
use binance_async::model::{Order as QueryOrder, Symbol};
use std::fmt::Debug;
use std::time::Duration;

#[derive(Debug)]
pub enum Event {
    SetMarketValue(String, Monetary),
    SetAssetBalance(String, Monetary),
    ExecutedOrder(QueryOrder, Monetary),
    Evaluate(i64),
}

pub type MarketData = Symbol;

#[async_trait]
pub trait Environment {
    async fn initialize(&mut self) -> Result<Vec<MarketData>, ()>;
    async fn poll(&mut self) -> Event;
    async fn order(&mut self, symbol: &str, order: Order) -> Result<(), ()>;
    async fn update_balances(&self, balances: Vec<(&AssetSymbol, Monetary)>);
}

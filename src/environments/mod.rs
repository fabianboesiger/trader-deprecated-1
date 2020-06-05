mod historical;

pub use historical::Historical;

use crate::economy::Monetary;
use async_trait::async_trait;
use std::time::Duration;

#[derive(Debug)]
pub enum Event {
    UpdateMarketValue(String, Monetary),
    UpdateTime(Duration),
}

#[async_trait]
pub trait Environment {
    async fn poll(&mut self) -> Event;
}

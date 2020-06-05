mod historical;

pub use historical::Historical;

use async_trait::async_trait;
use std::time::Duration;
use std::fmt::Debug;
use bigdecimal::Num;

#[derive(Debug)]
pub enum Event<N>
    where
        N: Num
{
    UpdateMarketValue(String, N),
    UpdateTime(Duration),
}

#[async_trait]
pub trait Environment<N>
    where
        N: Num
{
    async fn poll(&mut self) -> Event<N>;
}

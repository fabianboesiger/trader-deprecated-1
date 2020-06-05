use super::{Environment, Event};
use crate::economy::Monetary;
use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::PgPool;
use std::time::Duration;

struct MarketValueChange {
    symbol: String,
    value: Monetary,
    timestamp: i64,
}

pub struct Historical {
    timestamp: i64,
    buffer: Vec<MarketValueChange>,
    pool: PgPool,
}

impl Historical {
    pub async fn new(start_at: DateTime<Utc>) -> Historical {
        dotenv::dotenv().ok();
        Historical {
            timestamp: start_at.timestamp(),
            buffer: Vec::new(),
            pool: PgPool::new(&std::env::var("DATABASE_URL").unwrap())
                .await
                .unwrap(),
        }
    }
}

#[async_trait]
impl Environment for Historical {
    async fn poll(&mut self) -> Event {
        if self.buffer.is_empty() {
            self.buffer = sqlx::query_as!(
                MarketValueChange,
                "
                    SELECT symbol, value, timestamp * 60 AS timestamp
                    FROM tickers
                    WHERE timestamp >= $1::BIGINT / 60
                    AND timestamp < $1::BIGINT / 60 + 60
                    ORDER BY timestamp DESC
                ",
                self.timestamp
            )
            .fetch_all(&self.pool)
            .await
            .unwrap();
        }

        let timestamp = self.buffer.last().unwrap().timestamp;
        if timestamp > self.timestamp {
            let duration = Duration::from_secs((timestamp - self.timestamp) as u64);
            self.timestamp = timestamp;
            return Event::UpdateTime(duration);
        }

        let next = self.buffer.pop().unwrap();
        Event::UpdateMarketValue(next.symbol, next.value)
    }
}

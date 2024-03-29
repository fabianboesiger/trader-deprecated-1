use super::{Environment, Event, MarketData, Order};
use crate::economy::Market;
use crate::traders::Action;
use async_trait::async_trait;
use binance_async::Binance;
use sqlx::PgPool;
use std::time::Duration;

struct MarketValueChange {
    symbol: String,
    value: f64,
    timestamp: i64,
}

pub struct Historical {
    timestamp: i64,
    buffer: Vec<MarketValueChange>,
    pool: PgPool,
    binance: Binance,
    events: Vec<Event>,
}

impl Historical {
    pub async fn new(start_at: i64) -> Historical {
        dotenv::dotenv().ok();
        Historical {
            timestamp: start_at,
            buffer: Vec::new(),
            pool: PgPool::new(&std::env::var("DATABASE_URL").unwrap())
                .await
                .unwrap(),
            binance: Binance::with_credential(
                &std::env::var("BINANCE_API_KEY").unwrap(),
                &std::env::var("BINANCE_SECRET_KEY").unwrap(),
            ),
            events: vec![Event::SetAssetBalance(String::from("USDT"), 200.0)],
        }
    }
}

#[async_trait]
impl Environment for Historical {
    async fn initialize(&mut self) -> Result<Vec<MarketData>, ()> {
        let stats = self
            .binance
            .get_24h_price_stats_all()
            .unwrap()
            .await
            .unwrap();
        let mut symbols = Vec::new();
        for stat in stats {
            if stat.count >= 3600 {
                symbols.push(stat.symbol);
            }
        }

        let exchange_info = self.binance.get_exchange_info().unwrap().await.unwrap();
        let mut markets = Vec::new();
        for symbol in exchange_info.symbols {
            if symbols.contains(&symbol.symbol)
                && (symbol.base_asset == "USDT" || symbol.quote_asset == "USDT")
            {
                markets.push(symbol);
            }
        }

        Ok(markets)
    }

    async fn poll(&mut self) -> Event {
        if let Some(event) = self.events.pop() {
            return event;
        }

        if self.buffer.is_empty() {
            println!("fetching data ...");
            self.buffer = sqlx::query_as!(
                MarketValueChange,
                "
                    SELECT symbol, value, timestamp * 60 AS timestamp
                    FROM tickers
                    WHERE timestamp >= $1::BIGINT / 60
                    AND timestamp < $1::BIGINT / 60 + 60 * 24
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
            return Event::Evaluate(0);
        }

        let next = self.buffer.pop().unwrap();
        Event::SetMarketValue(next.symbol, next.value)
    }

    async fn order(&mut self, symbol: &str, order: Order) -> Result<(), ()> {
        Ok(())
    }
}

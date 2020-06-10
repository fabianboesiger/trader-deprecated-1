use super::{Environment, Event, MarketData, OrderData};
use crate::economy::Market;
use crate::traders::Action;
use async_trait::async_trait;
use bigdecimal::{BigDecimal, ToPrimitive};
use binance_async::{model::websocket::Subscription, Binance, BinanceWebsocket};
use sqlx::PgPool;
use std::time::{Duration, SystemTime};
use tokio::stream::StreamExt;

struct MarketValueChange {
    symbol: String,
    value: BigDecimal,
    timestamp: i64,
}

pub struct Simulated {
    timestamp: SystemTime,
    buffer: Vec<MarketValueChange>,
    pool: PgPool,
    binance: Binance,
    socket: BinanceWebsocket,
    events: Vec<Event>,
}

impl Simulated {
    pub async fn new() -> Simulated {
        dotenv::dotenv().ok();
        Simulated {
            timestamp: SystemTime::now(),
            buffer: Vec::new(),
            pool: PgPool::new(&std::env::var("DATABASE_URL").unwrap())
                .await
                .unwrap(),
            binance: Binance::with_credential(
                &std::env::var("BINANCE_API_KEY").unwrap(),
                &std::env::var("BINANCE_SECRET_KEY").unwrap(),
            ),
            events: vec![Event::SetAssetBalance(String::from("USDT"), 200.0)],
            socket: {
                let mut socket = BinanceWebsocket::default();
                socket.subscribe(Subscription::TickerAll).await.unwrap();
                socket
            },
        }
    }
}

#[async_trait]
impl Environment for Simulated {
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

        while let Some(msg) = self.socket.try_next().await.unwrap() {
            println!("{:?}", msg);
        }

        unreachable!();
    }

    async fn order(&mut self, order: OrderData) -> Result<(), ()> {
        Ok(())
    }
}

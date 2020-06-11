use super::{Environment, Event, MarketData};
use crate::economy::Market;
use crate::traders::{Action, Order};
use async_trait::async_trait;
use binance_async::{
    model::{
        Side,
        OrderType,
        TimeInForce,
        websocket::{Subscription, BinanceWebsocketMessage}
    },
    Binance,
    BinanceWebsocket
};
use sqlx::PgPool;
use std::time::{Duration, SystemTime};
use tokio::stream::StreamExt;

struct MarketValueChange {
    symbol: String,
    value: f64,
    timestamp: i64,
}

pub struct Simulated {
    timestamp: i64,
    realtime: bool,
    buffer: Vec<MarketValueChange>,
    pool: PgPool,
    binance: Binance,
    socket: BinanceWebsocket,
    events: Vec<Event>,
    open_orders: Vec<String>,
}

impl Simulated {
    pub async fn new() -> Simulated {
        dotenv::dotenv().ok();

        let pool = PgPool::new(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();
        let timestamp = sqlx::query!("
                SELECT MIN(timestamp) AS timestamp
                FROM tickers
            ")
            .fetch_one(&pool)
            .await
            .unwrap()
            .timestamp
            .unwrap();
        let binance = Binance::with_credential(
                &std::env::var("BINANCE_API_KEY").unwrap(),
                &std::env::var("BINANCE_SECRET_KEY").unwrap(),
            );

        Simulated {
            timestamp,
            realtime: false,
            buffer: Vec::new(),
            pool,
            binance,
            events: vec![Event::SetAssetBalance(String::from("USDT"), 200.0)],
            socket: BinanceWebsocket::default(),
            open_orders: Vec::new()
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
            if stat.count >= 24 * 60 * 60 / 5 {
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
        loop {
            if let Some(event) = self.events.pop() {
                return event;
            }

            let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64 - 1;

            if self.buffer.is_empty() {
                if self.realtime {
                    //println!("fetching realtime data ... at {}, fetching {}", self.timestamp, now);
                    if let Some(message) = self.socket.try_next().await.unwrap() {
                        if let BinanceWebsocketMessage::TickerAll(tickers) = message {
                            for ticker in tickers {
                                self.buffer.push(MarketValueChange {
                                    symbol: ticker.symbol,
                                    value: ticker.current_close,
                                    timestamp: now,
                                });
                            }
                        }
                    }        
                } else {
                    self.timestamp += 1;
                    // TODO: Find a proper switchover criterion.
                    if self.timestamp + 3 >= now {
                        println!("switching over to realtime data ...");
                        self.realtime = true;
                        self.socket.subscribe(Subscription::TickerAll).await.unwrap();
                        continue;
                    }

                    let to = std::cmp::min(3600, now - self.timestamp);
                    println!("fetching historical data ...  at {}, fetching [{}-{}]", self.timestamp, self.timestamp, self.timestamp + to - 1);
                    self.buffer = sqlx::query_as!(
                        MarketValueChange,
                        "
                            SELECT symbol, value, timestamp
                            FROM tickers
                            WHERE timestamp >= $1::BIGINT
                            AND timestamp < $1::BIGINT + $2::BIGINT
                            ORDER BY timestamp DESC
                        ",
                        self.timestamp,
                        to
                    )
                    .fetch_all(&self.pool)
                    .await
                    .unwrap();
                }
            }        

            if let Some(next) = self.buffer.pop() {
                if next.timestamp == self.timestamp {
                    return Event::SetMarketValue(next.symbol, next.value);
                } else {
                    self.timestamp += 1;
                    return Event::Evaluate(self.timestamp - 1);
                }
            }
            
        }
    }

    async fn order(&mut self, symbol: &str, order: Order) -> Result<(), ()> {
        /*let order_request = match order {
            Order::Limit(Action::Buy, quantity, value) => {
                Ok(OrderRequest {
                    symbol: String::from(symbol),
                    qty: quantity,
                    price: value,
                    order_side: "BUY",
                    order_type: "LIMIT",
                    time_in_force: "FOK",
                })
            }
            _ => { Err(()) }
        };
        self.binance.custom_order(order_request.unwrap());*/
        Ok(())
    }
}

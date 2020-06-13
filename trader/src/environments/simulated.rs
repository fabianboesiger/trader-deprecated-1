use super::{Environment, Event, MarketData};
use crate::economy::{Market, AssetSymbol, Monetary};
use crate::traders::{Action, Order};
use async_trait::async_trait;
use binance_async::{
    model::{
        Side,
        OrderType,
        TimeInForce,
        Order as QueryOrder,
        OrderRequest,
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
    orders: Vec<(Order, QueryOrder, SystemTime, bool)>,
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
            orders: Vec::new()
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
            if stat.count >= 24 * 60 {
                symbols.push(stat.symbol);
            }
        }


        let exchange_info = self.binance.get_exchange_info().unwrap().await.unwrap();

        let mut to_usdt = vec![String::from("USDT")];
        for symbol in &exchange_info.symbols {
            if symbols.contains(&symbol.symbol) {
                if symbol.base_asset == "USDT" && !to_usdt.contains(&symbol.quote_asset) {
                    to_usdt.push(symbol.quote_asset.clone());
                } else
                if symbol.quote_asset == "USDT" && !to_usdt.contains(&symbol.base_asset) {
                    to_usdt.push(symbol.base_asset.clone());
                }
            }
        }

        let mut markets = Vec::new();
        for symbol in exchange_info.symbols {
            if symbols.contains(&symbol.symbol) {
                if to_usdt.contains(&symbol.base_asset) && to_usdt.contains(&symbol.quote_asset) {
                    markets.push(symbol);
                }
            }
        }

        Ok(markets)
    }

    async fn poll(&mut self) -> Event {
        loop {
            if let Some(event) = self.events.pop() {
                return event;
            }

            for (order, order_status, timestamp, open) in &mut self.orders {
                if *open {
                    //*order_status = self.binance.order_status(&order_status.symbol, order_status.order_id).unwrap().await.unwrap();
                    *open = match order_status.status.as_str() {
                        "FILLED" | "CANCELED" | "REJECTED" | "EXPIRED" => false,
                        _ => true,
                    };
                    if !*open {
                        return Event::ExecutedOrder(order_status.clone(), order_status.executed_qty);
                    }
                    if SystemTime::now().duration_since(*timestamp).unwrap().as_secs() > 10 {
                        //self.binance.cancel_order(&order_status.symbol, order_status.order_id).unwrap().await.unwrap();
                    }
                }
            }

            let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64 - 1;

            if self.buffer.is_empty() {
                if self.realtime {
                    //println!("fetching realtime data ... at {}, fetching {}", self.timestamp, now);
                    if let Some(message) = self.socket.try_next().await.unwrap() {
                        if let BinanceWebsocketMessage::MiniTickerAll(tickers) = message {
                            for ticker in tickers {
                                self.buffer.push(MarketValueChange {
                                    symbol: ticker.symbol,
                                    value: ticker.close,
                                    timestamp: now,
                                });
                            }
                        }
                    }        
                } else {
                    self.timestamp += 1;
                    // TODO: Find a proper switchover criterion.
                    if self.timestamp + 10 >= now {
                        println!("switching over to realtime data ...");
                        self.realtime = true;
                        self.socket.subscribe(Subscription::MiniTickerAll).await.unwrap();
                        continue;
                    }

                    let to = std::cmp::min(3600 * 24, now - self.timestamp);
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
        
        /*sqlx::query!(
            "
                INSERT INTO trades (id, symbol, base, quote, price, quantity, side, type, time_in_force) VALUES
                ($1, $2, $3, $4, $5)
            ",
            ""
        )
        .execute(&self.pool)
        .await
        .unwrap();*/
        
        let order_request = match order {
            Order::Limit(Action::Buy, quantity, value) => {
                Ok(OrderRequest {
                    symbol: String::from(symbol),
                    qty: quantity,
                    price: value,
                    order_side: Side::Buy,
                    order_type: String::from("LIMIT"),
                    time_in_force: String::from("GTC"),
                })
            },
            Order::Limit(Action::Sell, quantity, value) => {
                Ok(OrderRequest {
                    symbol: String::from(symbol),
                    qty: quantity,
                    price: value,
                    order_side: Side::Sell,
                    order_type: String::from("LIMIT"),
                    time_in_force: String::from("GTC"),
                })
            },
            /*
            Order::Market(Action::Buy, quantity) => {
                Ok(OrderRequest {
                    symbol: String::from(symbol),
                    qty: quantity,
                    price: 0.0,
                    order_side: Side::Buy,
                    order_type: String::from("MARKET"),
                    time_in_force: String::from("GTC"),
                })
            },
            Order::Market(Action::Sell, quantity) => {
                Ok(OrderRequest {
                    symbol: String::from(symbol),
                    qty: quantity,
                    price: 0.0,
                    order_side: Side::Sell,
                    order_type: String::from("MARKET"),
                    time_in_force: String::from("GTC"),
                })
            }
            */
            _ => { Err(()) }
        };

        if let Ok(order_request) = order_request {
            let query_order = QueryOrder {
                symbol: String::from(symbol),
                order_id: 0,
                client_order_id: String::new(),
                price: order_request.price,
                orig_qty: order_request.qty,
                executed_qty: order_request.qty,
                status: String::from("FILLED"),
                time_in_force: order_request.time_in_force,
                type_name: order_request.order_type,
                side: order_request.order_side,
                stop_price: 0.0,
                iceberg_qty: String::new(),
                time: 0,
            };

            self.orders.push((order, query_order, SystemTime::now(), true));

            Ok(())
        } else {
            // TODO: Proper error handling.
            Ok(())
        }
    }

    async fn update_balances(&self, balances: Vec<(&AssetSymbol, Monetary)>) {
        // Build query.
        let mut sql = String::new();
        sql.push_str("INSERT INTO balances (symbol, balance) VALUES");
        for i in 0..balances.len() {
            sql.push_str(&format!(
                " (${}, ${}),",
                i * 2 + 1,
                i * 2 + 2
            ));
        }
        sql.pop();
        sql.push_str(" ON CONFLICT (symbol) DO UPDATE SET balance = EXCLUDED.balance");

        // Bind values to query.
        let mut query = sqlx::query(&sql);
        for balance in &balances {
            query = query.bind(balance.0.as_str());
            query = query.bind(balance.1);
        }
        query.execute(&self.pool).await.unwrap();
    }
}

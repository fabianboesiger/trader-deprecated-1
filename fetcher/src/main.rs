use binance_async::{
    model::websocket::{BinanceWebsocketMessage, Subscription},
    BinanceWebsocket,
};
use sqlx::PgPool;
use std::{error::Error, time::SystemTime};
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let mut socket = BinanceWebsocket::default();
    socket.subscribe(Subscription::TickerAll).await?;
    let pool = PgPool::new(&std::env::var("DATABASE_URL")?).await?;
    let mut timestamp: Option<i64> = None;

    // Throw away first few results for better timing.
    for _ in 0..3 {
        socket.try_next().await?;
    }

    while let Some(message) = socket.try_next().await? {
        if let BinanceWebsocketMessage::TickerAll(tickers) = message {
            timestamp = Some(if let Some(now) = timestamp {
                let next = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)?
                    .as_millis() as i64;
                let expected_next = now + 1000;
                // Check if something went wrong with the timing.
                assert!((next - expected_next).abs() < 1000);
                expected_next
            } else {
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)?
                    .as_millis() as i64
            });

            // Build query.
            let mut sql = String::from("INSERT INTO tickers (symbol, value, timestamp) VALUES");
            for i in 0..tickers.len() {
                sql.push_str(&format!(
                    " (${}, ${}, ${}),",
                    i * 3 + 1,
                    i * 3 + 2,
                    i * 3 + 3
                ));
            }
            sql.pop();

            // Bind values to query.
            let mut query = sqlx::query(&sql);
            for ticker in &tickers {
                query = query.bind(&ticker.symbol);
                query = query.bind(ticker.current_close);
                query = query.bind(timestamp.unwrap() / 1000);
            }
            query.execute(&pool).await?;
        }
    }

    Ok(())
}

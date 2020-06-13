use warp::{Rejection, Reply, Filter, http::StatusCode, ws::{Message, WebSocket}};
use std::convert::Infallible;
use futures::{FutureExt, StreamExt};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc
};
use tokio::sync::{mpsc, Mutex};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time;
use sqlx::PgPool;
use serde::Serialize;
use std::time::SystemTime;

static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);

type Listeners = Arc<Mutex<HashMap<usize, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let pool = PgPool::new(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let listeners: Listeners = Arc::new(Mutex::new(HashMap::new()));
    let listeners_clone = listeners.clone();

    let index = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file("server/public/index.html"));

    let l = warp::any().map(move || listeners_clone.clone());

    let socket = warp::path("socket")
        .and(warp::ws())
        .and(l)
        .map(|ws: warp::ws::Ws, listeners| {
            ws.on_upgrade(move |socket| handle_connection(socket, listeners))
        });

    let public = warp::fs::dir("server/public");

    tokio::join!(
        warp::serve(index.or(socket).or(public).recover(handle_rejection))
            .run(([127, 0, 0, 1], 8000)),
        stream_data(pool, listeners)
    );
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let mut code = StatusCode::INTERNAL_SERVER_ERROR;
    let mut message = "Internal Server Error";

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    }

    Ok(warp::reply::with_status(message, code))
}

async fn handle_connection(socket: WebSocket, listeners: Listeners) {
    let id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);
    let (socket_tx, mut socket_rx) = socket.split();
    let (tx, rx) = mpsc::unbounded_channel();

    tokio::task::spawn(rx.forward(socket_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("websocket send error: {}", e);
        }
    }));

    listeners.lock().await.insert(id, tx);

    while let Some(result) = socket_rx.next().await {
        let _message = match result {
            Ok(message) => message,
            Err(error) => {
                eprintln!("websocket recieve error: {}", error);
                break;
            }
        };
    }

    listeners.lock().await.remove(&id);
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    total: f64,
    balances: Vec<Balance>,
    seconds_running: i64,
    profit_per_day: f64
}


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Balance {
    symbol: String,
    balance: f64,
    usdt: f64,
}

async fn stream_data(pool: PgPool, listeners: Listeners) {
    let mut interval = time::interval(Duration::from_secs(1));

    loop {
        let balances = sqlx::query_as!(
            Balance,
            "
                SELECT balances.symbol, balances.balance, balances.balance * COALESCE(
                    (
                        SELECT value
                        FROM tickers
                        WHERE tickers.symbol = balances.symbol || 'USDT'
                        ORDER BY timestamp DESC
                        LIMIT 1
                    ),
                    (
                        SELECT 1.0 / value
                        FROM tickers
                        WHERE tickers.symbol = 'USDT' || balances.symbol
                        ORDER BY timestamp DESC
                        LIMIT 1
                    ),
                    1.0
                ) AS usdt
                FROM balances
                WHERE balance > 0
            ")
            .fetch_all(&pool)
            .await
            .unwrap();

        let total = balances.iter().map(|balance| balance.usdt).sum();

        let start_at = sqlx::query!(
            "
            SELECT timestamp
            FROM tickers
            ORDER BY timestamp ASC
            LIMIT 1
            ")
            .fetch_one(&pool)
            .await
            .unwrap()
            .timestamp;

        let seconds_running = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64 - start_at;

        let start_capital = 200.0;

        let profit_per_day = (total - start_capital) / start_capital / (seconds_running as f64 / 60.0 / 60.0 / 24.0);

        let data = Data {
            total,
            balances,
            seconds_running,
            profit_per_day
        };

        let json_data = serde_json::to_string(&data).unwrap();

        for (_id, tx) in listeners.lock().await.iter_mut() {
            tx.send(Ok(Message::text(&json_data))).ok();
        }

        interval.tick().await;
    }
}
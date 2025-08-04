use redis::{Client};
use redis::aio::MultiplexedConnection;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type RedisPool = Arc<Mutex<MultiplexedConnection>>;

pub async fn init_redis_pool(redis_url: &str) -> RedisPool {
    let client = Client::open(redis_url).expect("Failed to create Redis client");
    let conn = client
        .get_multiplexed_async_connection()
        .await
        .expect("Failed to connect to Redis");
    Arc::new(Mutex::new(conn))
}

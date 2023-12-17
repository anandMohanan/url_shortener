use std::env;

use bb8_redis::{bb8::Pool, RedisConnectionManager};
use dotenv::dotenv;
use redis::AsyncCommands;

pub type RedisPool = Pool<RedisConnectionManager>;
pub async fn connect() -> RedisPool {
    dotenv().ok();

    let redis_url = env::var("REDIS_URL");
    let redis_url = match redis_url {
        Ok(v) => v,
        Err(_e) => panic!("Env variable not found"),
    };
    let redis_manager = RedisConnectionManager::new(redis_url).unwrap();
    bb8_redis::bb8::Pool::builder()
        .build(redis_manager)
        .await
        .unwrap()
}

pub async fn set(key: &str, value: &str, pool: RedisPool) {
    let mut connection = pool.get().await.unwrap();
    let _: () = connection.set(key, value).await.unwrap();

}


pub async fn get(key: &str, pool: RedisPool)  -> Option<String>{
    let mut connection = pool.get().await.unwrap();
    connection.get(key).await.unwrap()

}



mod redis;

pub use redis::connect;
pub use redis::set;
pub use redis::get;
pub use redis::RedisPool;

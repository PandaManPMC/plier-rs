use fred::prelude::*;
use log::{warn};
use fred::clients::Transaction;

#[derive(Debug, Clone)]
pub struct RDS {
    pool : RedisPool,
}

// let config = RedisConfig {
//     fail_fast: true,
//     blocking: fred::types::Blocking::default(),
//     username: None,
//     password: Some("U$89aRedis#".to_string()),
//     server: ServerConfig::Centralized {
//     server: Server{
//         host: "35.240.170.217".into(),
//         port: 26379,
//     }
//     },
//     version: RespVersion::RESP2,
//     database: Some(10),
// };

pub async fn init_rds(url: &str, pool_size: usize) -> Result<RDS, RedisError> {
    let res_pool = init_pool(url, pool_size).await;

    return match res_pool {
        Err(ref e) => {
            warn!("init_rds {:?}", e);
            Err(e.clone())
        },
        Ok(pool) => {
            let rds = RDS{pool};
            Ok(rds)
        }
    }
}

pub async fn init_pool(url: &str, pool_size: usize) -> Result<RedisPool, RedisError> {
    let config = RedisConfig::from_url(url)?;
    let pool = Builder::from_config(config).build_pool(pool_size)?;
    pool.init().await?;
    return Ok(pool)
}

impl RDS {

    pub async fn set<P: std::marker::Send>(&self, key: &str, val:P) -> Result<(), RedisError> where RedisValue: From<P> {
        self.pool.set(key, val, None, None, false).await?;
        Ok(())
    }

    pub async fn set_expire<P: std::marker::Send>(&self, key: &str, val:P, expire_second: i64) -> Result<(), RedisError> where RedisValue: From<P> {
        self.pool.set(key, val, Some(Expiration::EX(expire_second)), None, false).await?;
        Ok(())
    }

    pub async fn get_string(&self, key: &str) -> Result<String, RedisError> {
        self.pool.get::<String, _>(key).await
    }

    pub async fn get_i64(&self, key: &str) -> Result<i64, RedisError> {
        self.pool.get::<i64, _>(key).await
    }

    pub async fn incr(&self, key: &str) -> Result<(), RedisError> {
        self.pool.incr(key).await
    }

    pub async fn del(&self, key: &str) -> Result<(), RedisError> {
        self.pool.del(key).await
    }

    pub async fn get_tx(&self) -> Transaction  {
        return self.pool.multi();
    }

    pub async fn get_pool(&self) -> &RedisPool{
        return &self.pool;
    }

}

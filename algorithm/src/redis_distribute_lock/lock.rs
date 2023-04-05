use redis::{self, Commands, Connection, RedisResult};
use uuid::Uuid;

// lua拓展
// luaRefresh = `if redis.call("get", KEYS[1]) == ARGV[1] then return redis.call("pexpire", KEYS[1], ARGV[2]) else return 0 end`
// luaRelease = `if redis.call("get", KEYS[1]) == ARGV[1] then return redis.call("del", KEYS[1]) else return 0 end`
// luaPTTL    = `if redis.call("get", KEYS[1]) == ARGV[1] then return redis.call("pttl", KEYS[1]) else return -3 end`

pub struct RedisDistributeLock {
    connection: Connection,
    key: String,
    lock_id: String,
}

impl RedisDistributeLock {
    pub fn new(connection: Connection, key: String) -> RedisDistributeLock {
        RedisDistributeLock {
            connection,
            key,
            lock_id: Uuid::new_v4().to_string(),
        }
    }

    pub fn acquire(&mut self) -> RedisResult<bool> {
        let result: RedisResult<i64> = self.connection.set_nx(&(self.key), &self.lock_id);
        match result {
            Ok(value) => {
                println!("==> lock ok: {}", value);
                Ok(value == 1)
            }
            Err(err) => {
                println!("==> lock err: {}", err);
                Ok(false)
            }
        }
    }

    pub fn release(&mut self) -> RedisResult<bool> {
        let script = r#"
           if redis.call("get", KEYS[1]) == ARGV[1] then return redis.call("del", KEYS[1]) else return 0 end
        "#;

        let result: RedisResult<u64> = redis::Script::new(script)
            .key(&[&self.key])
            .arg(&[&self.lock_id])
            .invoke(&mut self.connection);
        match result {
            Ok(value) => {
                println!("==> release ok: {}", value);
                Ok(value == 1)
            }
            Err(err) => {
                println!("==> release err: {}", err);
                Ok(false)
            }
        }
    }
}
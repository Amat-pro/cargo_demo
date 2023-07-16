pub use dp::erase_overlap_intervals::erase_overlap_intervals;
pub use dp::length_of_lis::length_of_lis;
pub use dp::min_path_sum::min_path_sum;
pub use dp::triangle_yanghui::generate_triangle_yanghui;
pub use redis_distribute_lock::lock::*;
pub use sorting::bubble_sort::bubble_sort;

mod dp;
mod redis_distribute_lock;
mod sorting;

#[cfg(test)]
mod tests {
    use crate::RedisDistributeLock;

    #[test]
    fn test_redis_lock() {
        let client = redis::Client::open("redis://:password@ip:6379/15").unwrap();
        let con = client.get_connection().unwrap();

        let key = "my-key-03".to_string();
        let mut locker: RedisDistributeLock = RedisDistributeLock::new(con, key);

        let lock_result = locker.acquire();
        match lock_result {
            Ok(value) => {
                if value {
                    println!("lock success");
                    let release_result = locker.release();
                    match release_result {
                        Ok(value) => {
                            if value {
                                println!("release success");
                            } else {
                                println!("release fail");
                            }
                        }
                        Err(_) => {
                            panic!("release err");
                        }
                    }
                } else {
                    println!("lock fail");
                }
            }
            Err(_) => {
                panic!("lock err");
            }
        }
    }
}

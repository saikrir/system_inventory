use std::io::Error;

use redis::{cmd, Client, Commands};

pub struct CacheClient<'a> {
    pub redis_client: &'a Client,
}
impl<'a> CacheClient<'a> {
    pub fn add_to_cache(&self, key: &str, value: &str) -> Result<bool, Error> {
        let conn_res = self.redis_client.get_connection();

        match conn_res {
            Ok(mut conn) => {
                return match cmd("SET").arg(&key).arg(value).exec(&mut conn) {
                    Ok(_) => {
                        println!("cache written {:#?}", key);
                        Ok(true)
                    }
                    Err(_err) => Err(Error::other("failed to set value")),
                };
            }
            Err(err) => {
                println!("Err -> {:#?}", err);
                Err(Error::other("failed to get connection"))
            }
        }
    }

    pub fn get_cached(&self, key: &str) -> Result<String, Error> {
        let conn_res = self.redis_client.get_connection();
        if conn_res.is_err() {
            Err(Error::other("failed to get connection"))
        } else {
            let mut conn = conn_res.unwrap();
            let res = conn.get(key);
            match res {
                Ok(value) => Ok(value),
                Err(err) => Err(Error::other("failed to set value")),
            }
        }
    }
}

//

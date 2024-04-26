use std::{env, process};
use once_cell::sync::OnceCell;
use redis::{Client, Connection};


static REDIS_CLIENT: OnceCell<Connection> = OnceCell::new();

pub fn init_redis_pool(){
    let redis_url = env::var("REDIS_URL").expect("🔥 There is no REDIS_URL configuration in the configuration file.");
    //创建Redis客户端
    let client = Client::open(redis_url).expect("🔥 Unable to open to Redis client.");
    //获取连接
    let mut conn_result = client.get_connection();
   let conn=  match conn_result {
        Ok(val)=>{
            println!("✅ Connection to the redis is successful!");
            val
        }
        Err(e)=>{
            println!("🔥 Unable to connect to Redis => {:?}", e);
            process::exit(1);
        }
    };
    let _ = REDIS_CLIENT.set(conn).is_ok();
}

pub fn get_redis_connect()->&'static Connection{
    let conn = REDIS_CLIENT.get().expect("🔥 There is no redis client connection in the redis global variable");
    conn
}
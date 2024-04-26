use std::{env, process};
use once_cell::sync::OnceCell;
use sqlx::{MySqlPool,mysql::MySqlPoolOptions};

static MYSQL_POOL: OnceCell<MySqlPool> = OnceCell::new();

pub async fn init_db_pool(){
   let database_url =env::var("DATABASE_URL").expect("🔥 The variable name is not available in the configuration file.");
   let result_pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await;
   let pool= match result_pool {
        Ok(mysql_pool)=>{
            println!("✅ Connection to the mysql is successful!");
            mysql_pool
        }
        Err(e)=>{
            println!("🔥 Failed to connect to the database: {:?}", e);
            process::exit(1);
        }
    };
    let _ = MYSQL_POOL.set(pool).is_ok();
}


// 获取连接池中的连接
pub  fn get_connect()->&'static MySqlPool{
  let pool = MYSQL_POOL.get().expect("🔥 There is no connection available in the connection pool.");
    pool
}

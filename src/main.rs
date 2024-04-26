mod utils;
mod db;
mod router;
mod service;
mod api;
mod model;

use std::env;
use actix_web::{HttpServer, web, App};
use dotenv::dotenv;
use utils::auth_middleware::AuthMiddleware;
use utils::log_middleware::LoggingMiddleware;
use db::mysql::init_db_pool;
use db::redis::init_redis_pool;
use crate::router::router::register_router;




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //初始化日志
    env_logger::init();
    //加载.env文件
    dotenv().ok();
    //获取文件上传的大小限制
    let size = env::var("MAX_UPLOAD_SIZE")
        .expect("🔥 There is no MAX_UPLOAD_SIZE variable in the configuration file.")
        .parse::<usize>().expect("🔥 Error of string to numeric");
    //初始化mysql数据库连接
    init_db_pool().await;
    //初始化redis连接
    init_redis_pool();
    //绑定服务和端口
    let server_port = env::var("SERVER_PORT").expect("🔥 There is no SERVER_PORT configuration in the configuration file.");
    HttpServer::new(move||{
       App::new()
           .app_data(web::PayloadConfig::new(size * 1024 * 1024))
           .wrap(LoggingMiddleware)
           .service(web::scope("/api").configure(register_router))
    }).bind(server_port)?
      .run()
      .await
}






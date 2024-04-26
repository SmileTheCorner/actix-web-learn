use actix_web::{get, Responder};
use crate::utils::response_result::ResponseResult;
use crate::model::index::Student;
use crate::db::mysql::get_connect;
use crate::model::song::Song;

#[get("/hello")]
async fn hello()->impl Responder{
    let conn = get_connect();
    let row:Vec<Song>= sqlx::query_as("select * from tb_song").fetch_all(conn).await.unwrap();
    ResponseResult::success(row)
}

#[get("/index")]
async fn index()->impl Responder{
    let student = Student{name:"张三".to_string(),age:23,address:"云南昆明".to_string()};
    ResponseResult::success(student)
}
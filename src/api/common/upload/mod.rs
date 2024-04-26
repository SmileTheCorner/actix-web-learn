use actix_multipart::Multipart;
use actix_web::{post, Responder};
use crate::utils::response_result::ResponseResult;
use crate::service::common::upload::Upload;

//文件上传
#[post("/upload")]
async fn upload_file(file:Multipart) -> impl Responder{
    let upload_service = Upload::new();
    let file = upload_service.upload(file).await;
    match file {
        Ok(val)=>ResponseResult::success(val),
        Err(e)=>ResponseResult::fail(e.to_string())
    }
}
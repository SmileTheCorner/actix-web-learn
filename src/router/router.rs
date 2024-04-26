use actix_web::web::{ServiceConfig,self};
use super::common::upload::UploadRouter;
use super::test::TestRouter;

//注册路由
pub fn register_router(sc:&mut ServiceConfig){
   sc.service(
       web::scope("")
           .service(TestRouter::get_router()) //测试路由
           .service(UploadRouter::get_router()) //文件上传
   );
}
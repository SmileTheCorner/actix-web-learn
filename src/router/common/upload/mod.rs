use actix_web::{Scope,web};
use crate::api::common::upload::upload_file;

pub struct UploadRouter {}

impl UploadRouter{
   pub fn get_router()->Scope{
       web::scope("").service(upload_file)
   }
}
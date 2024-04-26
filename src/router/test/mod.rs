use actix_web::{Scope,web};
use crate::api::test::{auth::generate_token,auth::validation,index::index,index::hello};

pub struct TestRouter{}

impl TestRouter{
    pub fn get_router()->Scope{
        web::scope("").service(generate_token).service(validation).service(index).service(hello)
    }
}
use std::collections::HashMap;
use actix_web::{get, Responder, web};
use crate::utils::jwt_util::{create_token, validate_token};
use crate::utils::response_result::ResponseResult;


#[get("/token")]
async fn generate_token()->impl Responder{
    let token = create_token(String::from("234"),String::from("张三"));
    let mut map:HashMap<String,String> = HashMap::new();
    map.insert("token".to_string(),token);
    ResponseResult::success(map)
}

#[get("/validation/{token}")]
async fn validation(token:web::Path<String>)->impl Responder{
    let result = validate_token(token.to_string());
    match result {
        Ok(data) => {
            let claims = serde_json::to_string(&data).unwrap();
            ResponseResult::success(claims)
        },
        Err(e) => ResponseResult::fail(e.to_string()),
    }
}
use crate::utils::jwt_util::{validate_token};
use actix_web::{dev::{Service,forward_ready,ServiceRequest, ServiceResponse, Transform},error,Error};
use std::future::{ready, Ready};
use futures_util::future::LocalBoxFuture;

/*
在中间件处理过程器有两步.
1. 中间件初始化, 下一个服务链中作为一个参数中间件工厂被调用.
2. 中间件的调用方法被正常的请求调用.
 */
pub struct AuthMiddleware;


/*
中间件工厂是来自 actix_service 包下的一个 `Transform` trait.
`S` - 下一个服务类型
`B` - 响应body类型
*/
impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = Auth<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(Auth { service }))
    }
}

pub struct Auth<S> {
    service: S,
}

impl<S,B> Service<ServiceRequest> for Auth<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;


    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth = req.headers().get("Authorization");
        match auth {
            Some(val) => {
                let token = val.to_str().unwrap().split("Bearer ").collect::<Vec<&str>>().pop().unwrap();
                //校验token
                let result = validate_token(token.to_string());
                if let Err(e) = result {
                    return Box::pin(async move {
                        Err(error::ErrorUnauthorized(e.to_string()))
                    })
                }
            },
            None => return Box::pin(async move {
                Err(error::ErrorUnauthorized("请携带token"))
            }),
        }

        //请求中如果携带了token，并且校验通过了，则继续处理请求
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}


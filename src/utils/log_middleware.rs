use std::{
    future::{ready, Ready},
    sync::Arc
};

use actix_http::h1;
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error,
};
use futures_util::future::LocalBoxFuture;

pub struct LoggingMiddleware;

impl<S: 'static, B> Transform<S, ServiceRequest> for LoggingMiddleware
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggingMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggingMiddlewareService {
            service: Arc::new(service),
        }))
    }
}

pub struct LoggingMiddlewareService<S> {
    service: Arc<S>,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddlewareService<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        Box::pin(async move {
            let body = req.extract::<web::Bytes>().await?;
            req.set_payload(bytes_to_payload(body));
            let res = svc.call(req).await?;
            Ok(res)
        })
    }
}

fn bytes_to_payload(buf: web::Bytes) -> dev::Payload {
    let (_, mut pl) = h1::Payload::create(true);
    pl.unread_data(buf);
    dev::Payload::from(pl)
}
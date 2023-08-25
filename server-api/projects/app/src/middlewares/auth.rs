use std::{
    future::{ready, Future, Ready},
    pin::Pin,
    rc::Rc,
};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, HttpMessage,
};

use crate::{
    core::{config::SecretKey, database::DbPool},
    repositories,
};

pub struct Auth;

impl<S: 'static, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        Box::pin(async move {
            // Unwrap because we know that we have the data, otherwise that something is wrong
            let db = req.app_data::<web::Data<DbPool>>().unwrap();
            let secret_key = req.app_data::<web::Data<SecretKey>>().unwrap();
            let headers = req.headers();

            let user =
                repositories::user::get_user_token_from_request(headers, db, secret_key).await?;

            req.extensions_mut().insert(user);

            let res = svc.call(req).await?;
            Ok(res)
        })
    }
}

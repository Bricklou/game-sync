use actix_session::SessionExt;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, HttpMessage,
};
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
    rc::Rc,
};

use crate::{app, core::errors::AppError, data::AppData};
use crate::{core::database::DbPool, repositories};

pub struct Guest;

impl<S: 'static, B> Transform<S, ServiceRequest> for Guest
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = GuestMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(GuestMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct GuestMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for GuestMiddleware<S>
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
        let session = req.get_session();

        Box::pin(async move {
            let user_id = session.get::<i32>("user_id")?;

            match user_id {
                Some(user_id) => {
                    // Unwrap because we know that we have the data, otherwise that something is wrong
                    let app_data = req.app_data::<web::Data<AppData>>().unwrap();
                    let user = repositories::user::get_user_from_id(&app_data.db, user_id).await?;

                    if user.is_some() {
                        return Err(AppError::Unauthorized.into());
                    }

                    Ok(svc.call(req).await?)
                }
                None => Ok(svc.call(req).await?),
            }
        })
    }
}

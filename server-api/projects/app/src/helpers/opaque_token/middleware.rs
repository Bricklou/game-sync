use std::{collections::HashMap, fmt, future::Future, pin::Pin, rc::Rc};

use actix_session::storage::{LoadError, SessionKey, SessionStore};
use actix_utils::future::{ready, Ready};
use actix_web::{
    body::MessageBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    HttpResponse,
};

use super::config::{Configuration, SessionOatMiddlewareBuilder};

#[derive(Clone)]
pub struct SessionOatMiddleware<Store: SessionStore> {
    storage_backend: Rc<Store>,
    configuration: Rc<Configuration>,
}

impl<Store: SessionStore> SessionOatMiddleware<Store> {
    /// Use [`SessionMiddleware::new`] to initialize the session framework using the default
    /// parameters.
    ///
    /// To create a new instance of [`SessionMiddleware`] you need to provide:
    /// - an instance of the session storage backend you wish to use (i.e. an implementation of
    ///   [`SessionStore`]);
    pub fn new(store: Store) -> Self {
        Self::builder(store).build()
    }

    /// A fluent API to configure [`SessionMiddleware`].
    ///
    /// It takes as input the two required inputs to create a new instance of [`SessionMiddleware`]:
    /// - an instance of the session storage backend you wish to use (i.e. an implementation of
    ///   [`SessionStore`]);
    /// - a secret key, to sign or encrypt the content of client-side session cookie.
    pub fn builder(store: Store) -> SessionOatMiddlewareBuilder<Store> {
        SessionOatMiddlewareBuilder::new(store, super::config::default_configuration())
    }

    pub(crate) fn from_parts(store: Store, configuration: Configuration) -> Self {
        Self {
            storage_backend: Rc::new(store),
            configuration: Rc::new(configuration),
        }
    }
}

impl<S, B, Store> Transform<S, ServiceRequest> for SessionOatMiddleware<Store>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
    Store: SessionStore + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = InnerSessionOatMiddleware<S, Store>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(InnerSessionOatMiddleware {
            service: Rc::new(service),
            configuration: Rc::clone(&self.configuration),
            storage_backend: Rc::clone(&self.storage_backend),
        }))
    }
}

#[non_exhaustive]
pub struct InnerSessionOatMiddleware<S, Store: SessionStore + 'static> {
    service: Rc<S>,
    configuration: Rc<Configuration>,
    storage_backend: Rc<Store>,
}

impl<S, B, Store> Service<ServiceRequest> for InnerSessionOatMiddleware<S, Store>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    Store: SessionStore + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    #[allow(clippy::type_complexity)]
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let storage_backend = Rc::clone(&self.storage_backend);
        let configuration = Rc::clone(&self.configuration);

        Box::pin(async move {
            let session_key = extract_session_key(&req);
            let (session_key, session_state) =
                load_session_state(session_key, storage_backend.as_ref()).await?;

            let mut res = service.call(req).await?;

            // TODO: process all the data

            Ok(res)
        })
    }
}

fn extract_session_key(req: &ServiceRequest) -> Option<SessionKey> {
    let headers = req.headers();
    let authorization = headers.get("authorization")?;

    // Get the authorization value, return None if it doesn't exist
    // otherwise strip the "Bearer " prefix
    // then try to parse it into a session key or return None
    if let Some(authorization) = authorization.to_str().ok() {
        let authorization = authorization.strip_prefix("Bearer ")?;

        return match authorization.to_string().try_into() {
            Ok(session_key) => Some(session_key),
            Err(_) => None,
        };
    }

    None
}

/// Short-hand to create an `actix_web::Error` instance that will result in an `Internal Server
/// Error` response while preserving the error root cause (e.g. in logs).
fn e500<E: fmt::Debug + fmt::Display + 'static>(err: E) -> actix_web::Error {
    // We do not use `actix_web::error::ErrorInternalServerError` because we do not want to
    // leak internal implementation details to the caller.
    //
    // `actix_web::error::ErrorInternalServerError` includes the error Display representation
    // as body of the error responses, leading to messages like "There was an issue persisting
    // the session state" reaching API clients. We don't want that, we want opaque 500s.
    actix_web::error::InternalError::from_response(
        err,
        HttpResponse::InternalServerError().finish(),
    )
    .into()
}

async fn load_session_state<Store: SessionStore>(
    session_key: Option<SessionKey>,
    storage_backend: &Store,
) -> Result<(Option<SessionKey>, HashMap<String, String>), actix_web::Error> {
    if let Some(session_key) = session_key {
        match storage_backend.load(&session_key).await {
            Ok(state) => {
                if let Some(state) = state {
                    Ok((Some(session_key), state))
                } else {
                    // We discard the existing session key given that the state attached to it can
                    // no longer be found (e.g. it expired or we suffered some data loss in the
                    // storage). Regenerating the session key will trigger the `save` workflow
                    // instead of the `update` workflow if the session state is modified during the
                    // lifecycle of the current request.

                    tracing::info!(
                        "No session state has been found for a valid session key, creating a new \
                        empty session."
                    );

                    Ok((None, HashMap::new()))
                }
            }

            Err(err) => match err {
                LoadError::Deserialization(err) => {
                    tracing::warn!(
                        error.message = %err,
                        error.cause_chain = ?err,
                        "Invalid session state, creating a new empty session."
                    );

                    Ok((Some(session_key), HashMap::new()))
                }

                LoadError::Other(err) => Err(e500(err)),
            },
        }
    } else {
        Ok((None, HashMap::new()))
    }
}

use std::{collections::HashMap, fmt, future::Future, pin::Pin, rc::Rc};

use actix_service::{forward_ready, Service, Transform};
use actix_utils::future::{ready, Ready};
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    HttpResponse,
};

use crate::{
    config::{default_configuration, Configuration, SessionMiddlewareBuilder, TtlExtensionPolicy},
    provider::TokenProvider,
    storage::{LoadError, SessionKey, SessionStore},
    Session, SessionStatus,
};

#[derive(Clone)]
pub struct SessionMiddleware<Store: SessionStore, Provider: TokenProvider> {
    storage_backend: Rc<Store>,
    token_provider: Rc<Provider>,
    configuration: Rc<Configuration>,
}

impl<Store: SessionStore, Provider: TokenProvider> SessionMiddleware<Store, Provider> {
    /// Use [`SessionMiddleware::new`] to initialize the session framework using the default
    /// parameters.
    ///
    /// To create a new instance of [`SessionMiddleware`] you need to provide:
    /// - an instance of the session storage backend you wish to use (i.e. an implementation of
    ///   [`SessionStore`]);
    /// - an instance of the token provider backend you wish to use (i.e. an implementation of
    ///   [`TokenProvider`]);
    pub fn new(store: Store, provider: Provider) -> Self {
        Self::builder(store, provider).build()
    }

    /// A fluent API to configure [`SessionMiddleware`].
    ///
    /// It takes as input the two required inputs to create a new instance of [`SessionMiddleware`]:
    /// - an instance of the session storage backend you wish to use (i.e. an implementation of
    ///   [`SessionStore`]);
    /// - an instance of the token provider backend you wish to use (i.e. an implementation of
    ///   [`TokenProvider`]);
    pub fn builder(store: Store, provider: Provider) -> SessionMiddlewareBuilder<Store, Provider> {
        SessionMiddlewareBuilder::new(store, provider, default_configuration())
    }

    pub(crate) fn from_parts(
        store: Store,
        provider: Provider,
        configuration: Configuration,
    ) -> Self {
        Self {
            storage_backend: Rc::new(store),
            token_provider: Rc::new(provider),
            configuration: Rc::new(configuration),
        }
    }
}

impl<S, B, Store, Provider> Transform<S, ServiceRequest> for SessionMiddleware<Store, Provider>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
    Store: SessionStore + 'static,
    Provider: TokenProvider + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Transform = InnerSessionMiddleware<S, Store, Provider>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(InnerSessionMiddleware {
            service: Rc::new(service),
            configuration: Rc::clone(&self.configuration),
            storage_backend: Rc::clone(&self.storage_backend),
            token_provider: Rc::clone(&self.token_provider),
        }))
    }
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

#[doc(hidden)]
#[non_exhaustive]
pub struct InnerSessionMiddleware<
    S,
    Store: SessionStore + 'static,
    Provider: TokenProvider + 'static,
> {
    service: Rc<S>,
    configuration: Rc<Configuration>,
    storage_backend: Rc<Store>,
    token_provider: Rc<Provider>,
}

impl<S, B, Store, Provider> Service<ServiceRequest> for InnerSessionMiddleware<S, Store, Provider>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error> + 'static,
    S::Future: 'static,
    Store: SessionStore + 'static,
    Provider: TokenProvider + 'static,
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
        let token_provider = Rc::clone(&self.token_provider);

        Box::pin(async move {
            let session_key = token_provider.extract_session_key(&req);
            let (session_key, session_state) =
                load_session_state(session_key, storage_backend.as_ref()).await?;

            Session::set_session(&mut req, session_key.clone(), session_state);

            let mut res = service.call(req).await?;
            let (status, session_state) = Session::get_changes(&mut res);

            match session_key {
                None => {
                    // we do not create an entry in the session store if there is no state attached to a fresh session
                    if !session_state.is_empty() {
                        let session_key = storage_backend
                            .save(session_state, &configuration.session.state_ttl)
                            .await
                            .map_err(e500)?;

                        token_provider
                            .set_session(res.response_mut().head_mut(), session_key)
                            .map_err(e500)?;
                    }
                }
                Some(session_key) => match status {
                    SessionStatus::Changed => {
                        let session_key = storage_backend
                            .update(session_key, session_state, &configuration.session.state_ttl)
                            .await
                            .map_err(e500)?;

                        token_provider
                            .set_session(res.response_mut().head_mut(), session_key)
                            .map_err(e500)?;
                    }

                    SessionStatus::Purged => {
                        storage_backend.delete(&session_key).await.map_err(e500)?;

                        token_provider
                            .delete_session(res.response_mut().head_mut())
                            .map_err(e500)?;
                    }

                    SessionStatus::Renewed => {
                        storage_backend.delete(&session_key).await.map_err(e500)?;

                        let session_key = storage_backend
                            .save(session_state, &configuration.session.state_ttl)
                            .await
                            .map_err(e500)?;

                        token_provider
                            .set_session(res.response_mut().head_mut(), session_key)
                            .map_err(e500)?;
                    }

                    SessionStatus::Unchanged => {
                        if matches!(
                            configuration.ttl_extension_policy,
                            TtlExtensionPolicy::OnEveryRequest
                        ) {
                            storage_backend
                                .update_ttl(&session_key, &configuration.session.state_ttl)
                                .await
                                .map_err(e500)?;

                            token_provider
                                .set_session_cookie_unchanged(
                                    res.response_mut().head_mut(),
                                    session_key,
                                )
                                .map_err(e500)?;
                        }
                    }
                },
            }

            Ok(res)
        })
    }
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

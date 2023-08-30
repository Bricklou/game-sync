use actix_web::cookie::time::Duration;

use crate::{provider::TokenProvider, storage::SessionStore, SessionMiddleware};

#[derive(Clone)]
pub(crate) struct Configuration {
    pub(crate) session: SessionConfiguration,
    pub(crate) ttl_extension_policy: TtlExtensionPolicy,
}

#[derive(Clone)]
pub(crate) struct SessionConfiguration {
    pub(crate) state_ttl: Duration,
}

/// Configuration for which events should trigger an extension of the time-to-live for your session.
///
/// If you are using a [`BrowserSession`], `TtlExtensionPolicy` controls how often the TTL of the
/// session state should be refreshed. The browser is in control of the lifecycle of the session
/// cookie.
///
/// If you are using a [`PersistentSession`], `TtlExtensionPolicy` controls both the expiration of
/// the session cookie and the TTL of the session state on the storage backend.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum TtlExtensionPolicy {
    /// The TTL is refreshed every time the server receives a request associated with a session.
    ///
    /// # Performance impact
    /// Refreshing the TTL on every request is not free. It implies a refresh of the TTL on the
    /// session state. This translates into a request over the network if you are using a remote
    /// system as storage backend (e.g. Redis). This impacts both the total load on your storage
    /// backend (i.e. number of queries it has to handle) and the latency of the requests served by
    /// your server.
    OnEveryRequest,

    /// The TTL is refreshed every time the session state changes or the session key is renewed.
    OnStateChanges,
}

pub(crate) const fn default_ttl() -> Duration {
    Duration::days(1)
}

pub(crate) const fn default_ttl_extension_policy() -> TtlExtensionPolicy {
    TtlExtensionPolicy::OnStateChanges
}

/// A fluent, customized [`SessionMiddleware`] builder.
#[must_use]
pub struct SessionMiddlewareBuilder<Store: SessionStore, Provider: TokenProvider> {
    storage_backend: Store,
    provider: Provider,
    configuration: Configuration,
}

impl<Store: SessionStore, Provider: TokenProvider> SessionMiddlewareBuilder<Store, Provider> {
    pub(crate) fn new(store: Store, provider: Provider, configuration: Configuration) -> Self {
        Self {
            storage_backend: store,
            provider,
            configuration,
        }
    }

    /// Finalise the builder and return a [`SessionMiddleware`] instance.
    #[must_use]
    pub fn build(self) -> SessionMiddleware<Store, Provider> {
        SessionMiddleware::from_parts(self.storage_backend, self.provider, self.configuration)
    }
}

pub(crate) fn default_configuration() -> Configuration {
    Configuration {
        session: SessionConfiguration {
            state_ttl: default_ttl(),
        },
        ttl_extension_policy: default_ttl_extension_policy(),
    }
}

use actix_session::storage::SessionStore;
use time::Duration;

use super::middleware::SessionOatMiddleware;

#[derive(Clone)]
pub struct Configuration {
    pub session: SessionConfiguration,
    pub ttl_extension_policy: TtlExtensionPolicy,
}

#[derive(Clone)]
pub struct SessionConfiguration {
    pub state_ttl: Duration,
}

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

#[must_use]
pub struct SessionOatMiddlewareBuilder<Store: SessionStore> {
    storage_backend: Store,
    configuration: Configuration,
}

impl<Store: SessionStore> SessionOatMiddlewareBuilder<Store> {
    pub fn new(store: Store, configuration: Configuration) -> Self {
        Self {
            storage_backend: store,
            configuration,
        }
    }

    /// TTL of the session state.
    pub fn session_ttl(mut self, session_ttl: Duration) -> Self {
        self.configuration.session.state_ttl = session_ttl;
        self
    }

    /// TTL extension policy.
    pub fn ttl_extension_policy(mut self, ttl_extension_policy: TtlExtensionPolicy) -> Self {
        self.configuration.ttl_extension_policy = ttl_extension_policy;
        self
    }

    /// Determines what type of session cookie should be used and how its lifecycle should be managed.
    /// Check out [`SessionLifecycle`]'s documentation for more details on the available options.
    ///
    /// Default is [`SessionLifecycle::BrowserSession`].
    ///
    /// # Examples
    /// ```
    /// use actix_web::cookie::{Key, time::Duration};
    /// use actix_session::{SessionMiddleware, config::PersistentSession};
    /// use actix_session::storage::CookieSessionStore;
    ///
    /// const SECS_IN_WEEK: i64 = 60 * 60 * 24 * 7;
    ///
    /// // creates a session middleware with a time-to-live (expiry) of 1 week
    /// SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
    ///     .session_lifecycle(
    ///         PersistentSession::default().session_ttl(Duration::seconds(SECS_IN_WEEK))
    ///     )
    ///     .build();
    /// ```
    pub fn session_lifecycle(
        mut self,
        session_ttl: Duration,
        ttl_extension_policy: TtlExtensionPolicy,
    ) -> Self {
        self.configuration.session.state_ttl = session_ttl;
        self.configuration.ttl_extension_policy = ttl_extension_policy;

        self
    }

    /// Finalise the builder and return a [`SessionMiddleware`] instance.
    #[must_use]
    pub fn build(self) -> SessionOatMiddleware<Store> {
        SessionOatMiddleware::from_parts(self.storage_backend, self.configuration)
    }
}

pub fn default_configuration() -> Configuration {
    Configuration {
        session: SessionConfiguration {
            state_ttl: default_ttl(),
        },
        ttl_extension_policy: default_ttl_extension_policy(),
    }
}

pub const fn default_ttl() -> Duration {
    Duration::days(1)
}

pub const fn default_ttl_extension_policy() -> TtlExtensionPolicy {
    TtlExtensionPolicy::OnStateChanges
}

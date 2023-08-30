use std::rc::Rc;

use actix_web::{
    cookie::{time::Duration, Cookie, CookieJar, Key, SameSite},
    dev::{ResponseHead, ServiceRequest},
    http::header::{HeaderValue, SET_COOKIE},
};
use anyhow::Context;

use crate::storage::SessionKey;

use super::TokenProvider;

#[derive(Clone)]
pub struct CookieTokenProvider {
    configuration: Rc<CookieConfiguration>,
}

impl CookieTokenProvider {
    /// A fluent API to configure [`CookieTokenProvider`].
    /// It takes as input the only required input to create a new instance of [`CookieTokenProvider`] - a
    /// key to encrypt the cookie
    pub fn builder(key: Key) -> CookieTokenProviderBuilder {
        CookieTokenProviderBuilder::new(default_cookie_configuration(key))
    }

    /// Create a new instance of [`CookieTokenProvider`] using the default configuration.
    /// It takes as input the only required input to create a new instance of [`CookieTokenProvider`] -
    /// key to encrypt the cookie.
    pub fn new(key: Key) -> CookieTokenProvider {
        Self::builder(key).build()
    }
}

/// Determines how to secure the content of the session cookie.
///
/// Used by [`SessionMiddlewareBuilder::cookie_content_security`].
#[derive(Debug, Clone, Copy)]
pub enum CookieContentSecurity {
    /// The cookie content is encrypted when using `CookieContentSecurity::Private`.
    ///
    /// Encryption guarantees confidentiality and integrity: the client cannot tamper with the
    /// cookie content nor decode it, as long as the encryption key remains confidential.
    Private,

    /// The cookie content is signed when using `CookieContentSecurity::Signed`.
    ///
    /// Signing guarantees integrity, but it doesn't ensure confidentiality: the client cannot
    /// tamper with the cookie content, but they can read it.
    Signed,
}

#[derive(Clone)]
pub(crate) struct CookieConfiguration {
    pub(crate) secure: bool,
    pub(crate) http_only: bool,
    pub(crate) name: String,
    pub(crate) same_site: SameSite,
    pub(crate) path: String,
    pub(crate) domain: Option<String>,
    pub(crate) max_age: Option<Duration>,
    pub(crate) content_security: CookieContentSecurity,
    pub(crate) key: Key,
}

/// A fluent builder to construct a [`RedisSessionStore`] instance with custom configuration
/// parameters.
///
/// [`RedisSessionStore`]: crate::storage::RedisSessionStore
#[must_use]
pub struct CookieTokenProviderBuilder {
    cookie_configuration: CookieConfiguration,
}

impl CookieTokenProviderBuilder {
    pub(crate) fn new(configuration: CookieConfiguration) -> Self {
        Self {
            cookie_configuration: configuration,
        }
    }

    //// Set the name of the cookie used to store the session ID.
    ///
    /// Defaults to `id`.
    pub fn cookie_name(mut self, name: String) -> Self {
        self.cookie_configuration.name = name;
        self
    }

    /// Set the `Secure` attribute for the cookie used to store the session ID.
    ///
    /// If the cookie is set as secure, it will only be transmitted when the connection is secure
    /// (using `https`).
    ///
    /// Default is `true`.
    pub fn cookie_secure(mut self, secure: bool) -> Self {
        self.cookie_configuration.secure = secure;
        self
    }

    /// Set the `SameSite` attribute for the cookie used to store the session ID.
    ///
    /// By default, the attribute is set to `Lax`.
    pub fn cookie_same_site(mut self, same_site: SameSite) -> Self {
        self.cookie_configuration.same_site = same_site;
        self
    }

    /// Set the `Path` attribute for the cookie used to store the session ID.
    ///
    /// By default, the attribute is set to `/`.
    pub fn cookie_path(mut self, path: String) -> Self {
        self.cookie_configuration.path = path;
        self
    }

    /// Set the `Domain` attribute for the cookie used to store the session ID.
    ///
    /// Use `None` to leave the attribute unspecified. If unspecified, the attribute defaults
    /// to the same host that set the cookie, excluding subdomains.
    ///
    /// By default, the attribute is left unspecified.
    pub fn cookie_domain(mut self, domain: Option<String>) -> Self {
        self.cookie_configuration.domain = domain;
        self
    }

    /// Choose how the session cookie content should be secured.
    ///
    /// - [`CookieContentSecurity::Private`] selects encrypted cookie content.
    /// - [`CookieContentSecurity::Signed`] selects signed cookie content.
    ///
    /// # Default
    /// By default, the cookie content is encrypted. Encrypted was chosen instead of signed as
    /// default because it reduces the chances of sensitive information being exposed in the session
    /// key by accident, regardless of [`SessionStore`] implementation you chose to use.
    ///
    /// For example, if you are using cookie-based storage, you definitely want the cookie content
    /// to be encrypted—the whole session state is embedded in the cookie! If you are using
    /// Redis-based storage, signed is more than enough - the cookie content is just a unique
    /// tamper-proof session key.
    pub fn cookie_content_security(mut self, content_security: CookieContentSecurity) -> Self {
        self.cookie_configuration.content_security = content_security;
        self
    }

    /// Set the `HttpOnly` attribute for the cookie used to store the session ID.
    ///
    /// If the cookie is set as `HttpOnly`, it will not be visible to any JavaScript snippets
    /// running in the browser.
    ///
    /// Default is `true`.
    pub fn cookie_http_only(mut self, http_only: bool) -> Self {
        self.cookie_configuration.http_only = http_only;
        self
    }

    /// Set the session ttl
    pub fn session_ttl(mut self, duration: Duration) -> Self {
        self.cookie_configuration.max_age = Some(duration);
        self
    }

    /// Finalise the builder and return a [`CookieTokenProvider`] instance.
    #[must_use]
    pub fn build(self) -> CookieTokenProvider {
        CookieTokenProvider {
            configuration: Rc::new(self.cookie_configuration),
        }
    }
}

pub(crate) fn default_cookie_configuration(key: Key) -> CookieConfiguration {
    CookieConfiguration {
        secure: true,
        http_only: true,
        name: "id".into(),
        same_site: SameSite::Lax,
        path: "/".into(),
        domain: None,
        max_age: None,
        content_security: CookieContentSecurity::Private,
        key,
    }
}

#[async_trait::async_trait(?Send)]
impl TokenProvider for CookieTokenProvider {
    fn extract_session_key(&self, req: &ServiceRequest) -> Option<SessionKey> {
        let cookies = req.cookies().ok()?;
        let session_cookie = cookies
            .iter()
            .find(|&cookie| cookie.name() == self.configuration.name)?;

        let mut jar = CookieJar::new();
        jar.add_original(session_cookie.clone());

        let verification_result = match self.configuration.content_security {
            CookieContentSecurity::Signed => jar
                .signed(&self.configuration.key)
                .get(&self.configuration.name),
            CookieContentSecurity::Private => jar
                .private(&self.configuration.key)
                .get(&self.configuration.name),
        };

        if verification_result.is_none() {
            tracing::warn!(
                "The session cookie attached to the incoming request failed to pass cryptographic \
                checks (signature verification/decryption)."
            );
        }

        match verification_result?.value().to_owned().try_into() {
            Ok(session_key) => Some(session_key),
            Err(err) => {
                tracing::warn!(
                    error.message = %err,
                    error.cause_chain = ?err,
                    "Invalid session key, ignoring."
                );

                None
            }
        }
    }

    fn set_session(
        &self,
        response: &mut ResponseHead,
        session_key: SessionKey,
    ) -> Result<(), anyhow::Error> {
        let value: String = session_key.into();
        let mut cookie = Cookie::new(self.configuration.name.clone(), value);

        cookie.set_secure(self.configuration.secure);
        cookie.set_http_only(self.configuration.http_only);
        cookie.set_same_site(self.configuration.same_site);
        cookie.set_path(self.configuration.path.clone());

        if let Some(max_age) = self.configuration.max_age {
            cookie.set_max_age(max_age);
        }

        if let Some(ref domain) = self.configuration.domain {
            cookie.set_domain(domain.clone());
        }

        let mut jar = CookieJar::new();
        match &self.configuration.content_security {
            CookieContentSecurity::Signed => jar.signed_mut(&self.configuration.key).add(cookie),
            CookieContentSecurity::Private => jar.private_mut(&self.configuration.key).add(cookie),
        }

        // set cookie
        let cookie = jar.delta().next().unwrap();
        let val = HeaderValue::from_str(&cookie.encoded().to_string())
            .context("Failed to attach a session cookie to the outgoing response")?;

        response.headers_mut().append(SET_COOKIE, val);

        Ok(())
    }

    fn delete_session(&self, response: &mut ResponseHead) -> Result<(), anyhow::Error> {
        let removal_cookie = Cookie::build(self.configuration.name.clone(), "")
            .path(self.configuration.path.clone())
            .secure(self.configuration.secure)
            .http_only(self.configuration.http_only)
            .same_site(self.configuration.same_site);

        let mut removal_cookie = if let Some(ref domain) = &self.configuration.domain {
            removal_cookie.domain(domain)
        } else {
            removal_cookie
        }
        .finish();

        removal_cookie.make_removal();

        let val = HeaderValue::from_str(&removal_cookie.to_string())
            .context("Failed to attach a session removal cookie to the outgoing response")?;
        response.headers_mut().append(SET_COOKIE, val);

        Ok(())
    }

    fn set_session_cookie_unchanged(
        &self,
        response: &mut ResponseHead,
        session_key: SessionKey,
    ) -> Result<(), anyhow::Error> {
        if self.configuration.max_age.is_none() {
            return Ok(());
        }

        self.set_session(response, session_key)
    }
}

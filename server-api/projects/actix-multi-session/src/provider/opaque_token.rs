use actix_web::{
    dev::{ResponseHead, ServiceRequest},
    http::header::{HeaderValue, WWW_AUTHENTICATE},
};
use anyhow::Context;

use crate::storage::SessionKey;

use super::TokenProvider;

#[derive(Clone)]
pub struct OpaqueTokenProvider {}

impl OpaqueTokenProvider {
    /// Create a new instance of [`OpaqueTokenProvider`] using the default configuration.
    /// It takes as input the only required input to create a new instance of [`OpaqueTokenProvider`] -
    /// key to encrypt the cookie.
    pub fn new() -> OpaqueTokenProvider {
        Self {}
    }
}

#[async_trait::async_trait(?Send)]
impl TokenProvider for OpaqueTokenProvider {
    fn extract_session_key(&self, req: &ServiceRequest) -> Option<SessionKey> {
        let headers = req.headers();
        let session_key = headers.get("Authorization")?.to_str().ok()?;
        let session_key = session_key.trim_start_matches("Bearer ");

        println!("Session key: {:?}", session_key);
        session_key.to_owned().try_into().ok()
    }

    fn set_session(
        &self,
        response: &mut ResponseHead,
        session_key: SessionKey,
    ) -> Result<(), anyhow::Error> {
        let value: String = session_key.into();

        let src = format!("Bearer {}", value);

        let val = HeaderValue::from_str(&src)
            .context("Failed to attach a session cookie to the outgoing response")?;

        response.headers_mut().append(WWW_AUTHENTICATE, val);

        Ok(())
    }

    fn delete_session(&self, _response: &mut ResponseHead) -> Result<(), anyhow::Error> {
        // Do nothing, unlike cookies, the headers is only sent once, so we don't need to delete it
        Ok(())
    }

    fn set_session_cookie_unchanged(
        &self,
        response: &mut ResponseHead,
        session_key: SessionKey,
    ) -> Result<(), anyhow::Error> {
        self.set_session(response, session_key)
    }
}

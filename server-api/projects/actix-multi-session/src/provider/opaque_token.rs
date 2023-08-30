use actix_web::dev::{ResponseHead, ServiceRequest};

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

        session_key.to_owned().try_into().ok()
    }

    fn set_session(
        &self,
        _response: &mut ResponseHead,
        _session_key: SessionKey,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn delete_session(&self, _response: &mut ResponseHead) -> Result<(), anyhow::Error> {
        Ok(())
    }

    fn set_session_cookie_unchanged(
        &self,
        _response: &mut ResponseHead,
        _session_key: SessionKey,
    ) -> Result<(), anyhow::Error> {
        Ok(())
    }
}

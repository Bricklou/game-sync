use actix_web::dev::{ResponseHead, ServiceRequest};

use crate::storage::SessionKey;

#[async_trait::async_trait(?Send)]
pub trait TokenProvider {
    fn extract_session_key(&self, req: &ServiceRequest) -> Option<SessionKey>;
    fn set_session(
        &self,
        response: &mut ResponseHead,
        session_key: SessionKey,
    ) -> Result<(), anyhow::Error>;
    fn delete_session(&self, response: &mut ResponseHead) -> Result<(), anyhow::Error>;

    fn set_session_cookie_unchanged(
        &self,
        response: &mut ResponseHead,
        session_key: SessionKey,
    ) -> Result<(), anyhow::Error>;
}

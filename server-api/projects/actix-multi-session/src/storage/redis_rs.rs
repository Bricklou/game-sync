use std::{convert::TryInto, sync::Arc};

use actix_web::cookie::time::Duration;
use anyhow::{Context, Error};
use redis::{aio::ConnectionManager, AsyncCommands, Cmd, FromRedisValue, RedisResult, Value};

use super::SessionKey;
use crate::storage::{
    interface::{LoadError, SaveError, SessionState, UpdateError},
    utils::generate_session_key,
    SessionStore,
};

#[derive(Clone)]
pub struct RedisSessionStore {
    configuration: CacheConfiguration,
    client: ConnectionManager,
}

#[derive(Clone)]
struct CacheConfiguration {
    cache_keygen: Arc<dyn Fn(&str) -> String + Send + Sync>,
}

impl Default for CacheConfiguration {
    fn default() -> Self {
        Self {
            cache_keygen: Arc::new(str::to_owned),
        }
    }
}

impl RedisSessionStore {
    /// A fluent API to configure [`RedisSessionStore`].
    /// It takes as input the only required input to create a new instance of [`RedisSessionStore`] - a
    /// connection string for Redis.
    pub fn builder<S: Into<String>>(connection_string: S) -> RedisSessionStoreBuilder {
        RedisSessionStoreBuilder {
            configuration: CacheConfiguration::default(),
            connection_string: connection_string.into(),
        }
    }

    /// Create a new instance of [`RedisSessionStore`] using the default configuration.
    /// It takes as input the only required input to create a new instance of [`RedisSessionStore`] - a
    /// connection string for Redis.
    pub async fn new<S: Into<String>>(
        connection_string: S,
    ) -> Result<RedisSessionStore, anyhow::Error> {
        Self::builder(connection_string).build().await
    }
}

/// A fluent builder to construct a [`RedisSessionStore`] instance with custom configuration
/// parameters.
///
/// [`RedisSessionStore`]: crate::storage::RedisSessionStore
#[must_use]
pub struct RedisSessionStoreBuilder {
    connection_string: String,
    configuration: CacheConfiguration,
}

impl RedisSessionStoreBuilder {
    /// Set a custom cache key generation strategy, expecting a session key as input.
    pub fn cache_keygen<F>(mut self, keygen: F) -> Self
    where
        F: Fn(&str) -> String + 'static + Send + Sync,
    {
        self.configuration.cache_keygen = Arc::new(keygen);
        self
    }

    /// Finalise the builder and return a [`RedisActorSessionStore`] instance.
    ///
    /// [`RedisActorSessionStore`]: crate::storage::RedisActorSessionStore
    pub async fn build(self) -> Result<RedisSessionStore, anyhow::Error> {
        let client = ConnectionManager::new(redis::Client::open(self.connection_string)?).await?;
        Ok(RedisSessionStore {
            configuration: self.configuration,
            client,
        })
    }
}

#[async_trait::async_trait(?Send)]
impl SessionStore for RedisSessionStore {
    async fn load(&self, session_key: &SessionKey) -> Result<Option<SessionState>, LoadError> {
        let cache_key = (self.configuration.cache_keygen)(session_key.as_ref());

        let value: Option<String> = self
            .execute_command(redis::cmd("GET").arg(&[&cache_key]))
            .await
            .map_err(Into::into)
            .map_err(LoadError::Other)?;

        match value {
            None => Ok(None),
            Some(value) => Ok(serde_json::from_str(&value)
                .map_err(Into::into)
                .map_err(LoadError::Deserialization)?),
        }
    }

    async fn save(
        &self,
        session_state: SessionState,
        ttl: &Duration,
    ) -> Result<SessionKey, SaveError> {
        let body = serde_json::to_string(&session_state)
            .map_err(Into::into)
            .map_err(SaveError::Serialization)?;
        let session_key = generate_session_key();
        let cache_key = (self.configuration.cache_keygen)(session_key.as_ref());

        self.execute_command(redis::cmd("SET").arg(&[
            &cache_key,
            &body,
            "NX", // NX: only set the key if it does not already exist
            "EX", // EX: set expiry
            &format!("{}", ttl.whole_seconds()),
        ]))
        .await
        .map_err(Into::into)
        .map_err(SaveError::Other)?;

        Ok(session_key)
    }

    async fn update(
        &self,
        session_key: SessionKey,
        session_state: SessionState,
        ttl: &Duration,
    ) -> Result<SessionKey, UpdateError> {
        let body = serde_json::to_string(&session_state)
            .map_err(Into::into)
            .map_err(UpdateError::Serialization)?;

        let cache_key = (self.configuration.cache_keygen)(session_key.as_ref());

        let v: redis::Value = self
            .execute_command(redis::cmd("SET").arg(&[
                &cache_key,
                &body,
                "XX", // XX: Only set the key if it already exist.
                "EX", // EX: set expiry
                &format!("{}", ttl.whole_seconds()),
            ]))
            .await
            .map_err(Into::into)
            .map_err(UpdateError::Other)?;

        match v {
            Value::Nil => {
                // The SET operation was not performed because the XX condition was not verified.
                // This can happen if the session state expired between the load operation and the
                // update operation. Unlucky, to say the least. We fall back to the `save` routine
                // to ensure that the new key is unique.
                self.save(session_state, ttl)
                    .await
                    .map_err(|err| match err {
                        SaveError::Serialization(err) => UpdateError::Serialization(err),
                        SaveError::Other(err) => UpdateError::Other(err),
                    })
            }
            Value::Int(_) | Value::Okay | Value::Status(_) => Ok(session_key),
            val => Err(UpdateError::Other(anyhow::anyhow!(
                "Failed to update session state. {:?}",
                val
            ))),
        }
    }

    async fn update_ttl(&self, session_key: &SessionKey, ttl: &Duration) -> Result<(), Error> {
        let cache_key = (self.configuration.cache_keygen)(session_key.as_ref());

        self.client
            .clone()
            .expire(
                &cache_key,
                ttl.whole_seconds().try_into().context(
                    "Failed to convert the state TTL into the number of whole seconds remaining",
                )?,
            )
            .await?;
        Ok(())
    }

    async fn delete(&self, session_key: &SessionKey) -> Result<(), anyhow::Error> {
        let cache_key = (self.configuration.cache_keygen)(session_key.as_ref());
        self.execute_command(redis::cmd("DEL").arg(&[&cache_key]))
            .await
            .map_err(Into::into)
            .map_err(UpdateError::Other)?;

        Ok(())
    }
}

impl RedisSessionStore {
    /// Execute Redis command and retry once in certain cases.
    ///
    /// `ConnectionManager` automatically reconnects when it encounters an error talking to Redis.
    /// The request that bumped into the error, though, fails.
    ///
    /// This is generally OK, but there is an unpleasant edge case: Redis client timeouts. The
    /// server is configured to drop connections who have been active longer than a pre-determined
    /// threshold. `redis-rs` does not proactively detect that the connection has been dropped - you
    /// only find out when you try to use it.
    ///
    /// This helper method catches this case (`.is_connection_dropped`) to execute a retry. The
    /// retry will be executed on a fresh connection, therefore it is likely to succeed (or fail for
    /// a different more meaningful reason).
    async fn execute_command<T: FromRedisValue>(&self, cmd: &mut Cmd) -> RedisResult<T> {
        let mut can_retry = true;

        loop {
            match cmd.query_async(&mut self.client.clone()).await {
                Ok(value) => return Ok(value),
                Err(err) => {
                    if can_retry && err.is_connection_dropped() {
                        tracing::debug!(
                            "Connection dropped while trying to talk to Redis. Retrying."
                        );

                        // Retry at most once
                        can_retry = false;

                        continue;
                    } else {
                        return Err(err);
                    }
                }
            }
        }
    }
}

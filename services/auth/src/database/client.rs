//! Database client trait and implementation.

use chrono::{DateTime, Utc};
use deadpool_postgres::Pool;
use tonic::async_trait;
use uuid::Uuid;

use super::model::{OAuthAccount, Session};
use crate::error::DBError;
use crate::proto::OauthProvider;

#[cfg_attr(test, mock::db_client)]
#[async_trait]
pub trait DBClient: Send + Sync + 'static {
    async fn insert_session(&self, session: Session) -> Result<(), DBError>;

    async fn get_session(&self, id: &str) -> Result<Session, DBError>;

    async fn delete_session(&self, id: &str) -> Result<(), DBError>;

    async fn update_session(&self, id: &str, expires_at: &DateTime<Utc>) -> Result<(), DBError>;

    async fn upsert_oauth_account(
        &self,
        oauth_account: &OAuthAccount,
    ) -> Result<OAuthAccount, DBError>;

    async fn update_oauth_account(&self, id: &str, user_id: Uuid) -> Result<OAuthAccount, DBError>;

    async fn get_oauth_account(
        &self,
        user_id: Uuid,
        provider: OauthProvider,
    ) -> Result<OAuthAccount, DBError>;
}

#[derive(Clone)]
pub struct PostgresDBClient {
    pub pool: Pool,
}

impl PostgresDBClient {
    #[must_use]
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DBClient for PostgresDBClient {
    async fn insert_session(&self, session: Session) -> Result<(), DBError> {
        self.insert_session(session).await
    }

    async fn get_session(&self, id: &str) -> Result<Session, DBError> {
        self.get_session(id).await
    }

    async fn delete_session(&self, id: &str) -> Result<(), DBError> {
        self.delete_session(id).await
    }

    async fn update_session(&self, id: &str, expires_at: &DateTime<Utc>) -> Result<(), DBError> {
        self.update_session(id, expires_at).await
    }

    async fn upsert_oauth_account(
        &self,
        oauth_account: &OAuthAccount,
    ) -> Result<OAuthAccount, DBError> {
        self.upsert_oauth_account(oauth_account).await
    }

    async fn update_oauth_account(&self, id: &str, user_id: Uuid) -> Result<OAuthAccount, DBError> {
        self.update_oauth_account(id, user_id).await
    }

    async fn get_oauth_account(
        &self,
        user_id: Uuid,
        provider: OauthProvider,
    ) -> Result<OAuthAccount, DBError> {
        self.get_oauth_account(user_id, provider).await
    }
}

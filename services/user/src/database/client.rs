//! Database client trait and implementation.

use deadpool_postgres::Pool;
use std::fmt::Debug;
use tonic::async_trait;
use uuid::Uuid;

use super::model::User;
use crate::error::DBError;

#[cfg_attr(test, mock::db_client)]
#[async_trait]
pub trait DBClient: Send + Sync + 'static {
    async fn insert_user(&self, user: &User) -> Result<(), DBError>;

    async fn get_user(&self, id: Uuid) -> Result<User, DBError>;
}

#[derive(Clone, Debug)]
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
    async fn insert_user(&self, user: &User) -> Result<(), DBError> {
        self.insert_user(user).await
    }

    async fn get_user(&self, id: Uuid) -> Result<User, DBError> {
        self.get_user(id).await
    }
}

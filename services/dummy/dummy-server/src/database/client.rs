//! Database client trait and implementation.

use deadpool_postgres::Pool;
use std::fmt::Debug;
use tonic::async_trait;
use uuid::Uuid;

use super::model::Entity;
use crate::error::DBError;

#[cfg_attr(test, mock::db_client)]
#[async_trait]
pub trait DBClient: Send + Sync + 'static {
    async fn insert_entity(&self, entity: &Entity) -> Result<(), DBError>;

    async fn get_entity(&self, id: Uuid, user_id: Uuid) -> Result<Entity, DBError>;
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
    async fn insert_entity(&self, entity: &Entity) -> Result<(), DBError> {
        self.insert_entity(entity).await
    }

    async fn get_entity(&self, id: Uuid, user_id: Uuid) -> Result<Entity, DBError> {
        self.get_entity(id, user_id).await
    }
}

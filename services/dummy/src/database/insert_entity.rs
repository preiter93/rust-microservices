//! Insert entity database operation.

use super::model::Entity;
use crate::error::DBError;

use super::client::PostgresDBClient;

impl PostgresDBClient {
    /// Inserts an entity into the database.
    ///
    /// # Errors
    /// - if the database connection cannot be established
    /// - if the database query fails
    pub(super) async fn insert_entity(&self, entity: &Entity) -> Result<(), DBError> {
        let client = self.pool.get().await?;

        client
            .execute(
                "INSERT INTO entities (id, user_id, name) VALUES ($1, $2, $3)",
                &[&entity.id, &entity.user_id, &entity.name],
            )
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::database::test_helpers::run_db_test;
    use crate::fixture::fixture_entity;

    #[tokio::test]
    async fn test_insert_entity() {
        run_db_test(|db_client| async move {
            let entity = fixture_entity(|e| {
                e.name = "test-insert".to_string();
            });

            let result = db_client.insert_entity(&entity).await;
            assert!(result.is_ok());
        })
        .await;
    }
}

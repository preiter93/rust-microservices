//! Get entity database operation.

use uuid::Uuid;

use super::model::Entity;
use crate::error::DBError;

use super::client::PostgresDBClient;

impl PostgresDBClient {
    /// Gets an entity from the database by id and user_id.
    ///
    /// # Errors
    /// - if the database connection cannot be established
    /// - if the database query fails
    /// - if the entity is not found
    pub(super) async fn get_entity(&self, id: Uuid, user_id: Uuid) -> Result<Entity, DBError> {
        let client = self.pool.get().await?;

        let stmt = client
            .prepare("SELECT id, user_id, name FROM entities WHERE id = $1 AND user_id = $2")
            .await?;
        let row = client.query_opt(&stmt, &[&id, &user_id]).await?;
        let Some(row) = row else {
            return Err(DBError::NotFound);
        };

        Ok(Entity::try_from(row)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::test_helpers::DbTestBuilder;
    use crate::fixture::{fixture_entity, fixture_uuid};
    use rstest::rstest;

    #[rstest]
    #[case::happy_path(
        fixture_uuid(),
        vec![fixture_entity(|_| {})],
        Ok(fixture_entity(|_| {}))
    )]
    #[case::not_found(
        Uuid::parse_str("99999999-9999-9999-9999-999999999999").unwrap(),
        vec![],
        Err(DBError::NotFound)
    )]
    #[tokio::test]
    async fn test_get_entity(
        #[case] entity_id: Uuid,
        #[case] given_entities: Vec<Entity>,
        #[case] want: Result<Entity, DBError>,
    ) {
        DbTestBuilder::new()
            .with_entities(given_entities)
            .run(|db_client| async move {
                let user_id = fixture_uuid();
                let got = db_client.get_entity(entity_id, user_id).await;

                match (got, want) {
                    (Ok(got_entity), Ok(want_entity)) => assert_eq!(got_entity, want_entity),
                    (Err(got_err), Err(want_err)) => {
                        assert_eq!(format!("{got_err}"), format!("{want_err}"))
                    }
                    (got, want) => panic!("expected {want:?}, got {got:?}"),
                }
            })
            .await;
    }
}

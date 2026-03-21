use crate::error::DBError;
use crate::model::Entity;
use deadpool_postgres::Pool;
use std::fmt::Debug;
use tokio_postgres::Row;
use tonic::async_trait;
use uuid::Uuid;

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
    /// # Errors
    /// - if the database connection cannot be established
    /// - if the database query fails
    async fn insert_entity(&self, entity: &Entity) -> Result<(), DBError> {
        let client = self.pool.get().await?;

        client
            .execute(
                "INSERT INTO entities (id, user_id, name) VALUES ($1, $2, $3)",
                &[&entity.id, &entity.user_id, &entity.name],
            )
            .await?;

        Ok(())
    }

    /// # Errors
    /// - if the database connection cannot be established
    /// - if the database query fails
    /// - If the entity is not found
    async fn get_entity(&self, id: Uuid, user_id: Uuid) -> Result<Entity, DBError> {
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

impl TryFrom<Row> for Entity {
    type Error = DBError;

    fn try_from(row: Row) -> Result<Self, DBError> {
        let id: Uuid = row.try_get("id")?;
        let user_id: Uuid = row.try_get("user_id")?;
        let name: String = row.try_get("name")?;

        Ok(Entity { id, user_id, name })
    }
}

#[cfg(test)]
pub mod test {
    pub(crate) use super::MockDBClient;
    use super::*;
    use crate::SERVICE_NAME;
    use crate::error::DBError;
    use crate::fixture::{fixture_entity, fixture_uuid};
    use crate::model::Entity;
    use rstest::rstest;
    use testutils::get_test_db;
    use uuid::Uuid;

    async fn run_db_test<F, Fut>(given_entities: Vec<Entity>, test_fn: F)
    where
        F: FnOnce(PostgresDBClient) -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        let migrations = std::fs::canonicalize("./migrations").unwrap();
        let pool = get_test_db(SERVICE_NAME, migrations)
            .await
            .expect("failed to get connection to test db");
        let db_client = PostgresDBClient { pool };

        for entity in &given_entities {
            db_client
                .insert_entity(entity)
                .await
                .expect("failed to insert entity");
        }

        test_fn(db_client).await;
    }

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
        run_db_test(given_entities, |db_client| async move {
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

//! Shared test helpers for database tests.

use super::client::PostgresDBClient;
use super::model::Entity;
use dummy::SERVICE_NAME;
use testutils::get_test_db;

pub struct DbTestBuilder {
    entities: Vec<Entity>,
}

impl DbTestBuilder {
    pub fn new() -> Self {
        Self { entities: vec![] }
    }

    pub fn with_entities(mut self, entities: Vec<Entity>) -> Self {
        self.entities = entities;
        self
    }

    pub async fn run<F, Fut>(self, test_fn: F)
    where
        F: FnOnce(PostgresDBClient) -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        let migrations = std::fs::canonicalize("./migrations").unwrap();
        let pool = get_test_db(SERVICE_NAME, migrations)
            .await
            .expect("failed to get connection to test db");
        let db_client = PostgresDBClient { pool };

        for entity in self.entities {
            db_client
                .insert_entity(&entity)
                .await
                .expect("failed to seed entity");
        }

        test_fn(db_client).await;
    }
}

/// Simple helper for tests that don't need seeding.
pub async fn run_db_test<F, Fut>(test_fn: F)
where
    F: FnOnce(PostgresDBClient) -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    DbTestBuilder::new().run(test_fn).await;
}

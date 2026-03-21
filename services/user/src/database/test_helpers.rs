//! Shared test helpers for database tests.

use super::client::PostgresDBClient;
use super::model::User;
use testutils::get_test_db;
use user::SERVICE_NAME;

pub struct DbTestBuilder {
    users: Vec<User>,
}

impl DbTestBuilder {
    pub fn new() -> Self {
        Self { users: vec![] }
    }

    pub fn with_users(mut self, users: Vec<User>) -> Self {
        self.users = users;
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

        for user in self.users {
            db_client
                .insert_user(&user)
                .await
                .expect("failed to seed user");
        }

        test_fn(db_client).await;
    }
}

//! Shared test helpers for database tests.

use super::client::PostgresDBClient;
use super::model::{OAuthAccount, Session};
use auth::SERVICE_NAME;
use testutils::get_test_db;

pub struct DbTestBuilder {
    sessions: Vec<Session>,
    accounts: Vec<OAuthAccount>,
}

impl DbTestBuilder {
    pub fn new() -> Self {
        Self {
            sessions: vec![],
            accounts: vec![],
        }
    }

    pub fn with_sessions(mut self, sessions: Vec<Session>) -> Self {
        self.sessions = sessions;
        self
    }

    pub fn with_accounts(mut self, accounts: Vec<OAuthAccount>) -> Self {
        self.accounts = accounts;
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

        for session in self.sessions {
            db_client
                .insert_session(session)
                .await
                .expect("failed to seed session");
        }

        for account in self.accounts {
            db_client
                .upsert_oauth_account(&account)
                .await
                .expect("failed to seed oauth account");
        }

        test_fn(db_client).await;
    }
}

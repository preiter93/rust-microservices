//! Insert user database operation.

use super::model::User;
use crate::error::DBError;

use super::client::PostgresDBClient;

impl PostgresDBClient {
    /// Inserts a user into the database.
    ///
    /// # Errors
    /// - if the database connection cannot be established
    /// - if the database query fails
    pub(super) async fn insert_user(&self, user: &User) -> Result<(), DBError> {
        let client = self.pool.get().await?;

        client
            .execute(
                "INSERT INTO users (id, name, email) VALUES ($1, $2, $3)",
                &[&user.id, &user.name, &user.email],
            )
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{database::test_helpers::DbTestBuilder, fixture::fixture_user};

    #[tokio::test]
    async fn test_insert_user() {
        DbTestBuilder::new()
            .run(|db_client| async move {
                let user = fixture_user(|u| {
                    u.name = "test-insert".to_string();
                });

                let result = db_client.insert_user(&user).await;
                assert!(result.is_ok());
            })
            .await;
    }
}

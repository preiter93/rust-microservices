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
    use crate::database::test_helpers::run_db_test;
    use crate::fixture::fixture_user;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_insert_user() {
        run_db_test(|db_client| async move {
            let user = fixture_user(|u| {
                u.id = Uuid::new_v4();
                u.name = "test-insert".to_string();
            });

            let result = db_client.insert_user(&user).await;
            assert!(result.is_ok());
        })
        .await;
    }
}

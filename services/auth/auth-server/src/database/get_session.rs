//! Get session database operation.

use super::client::PostgresDBClient;
use super::model::Session;
use crate::error::DBError;

impl PostgresDBClient {
    /// Returns a session from the database.
    ///
    /// # Errors
    /// - not found
    /// - database connection cannot be established
    /// - executing database statement fails
    pub(super) async fn get_session(&self, id: &str) -> Result<Session, DBError> {
        let client = self.pool.get().await?;

        let stmt = client
            .prepare("SELECT id, secret_hash, created_at, expires_at, user_id FROM sessions WHERE id = $1")
            .await?;
        let row = client.query_opt(&stmt, &[&id]).await?;
        let Some(row) = row else {
            return Err(DBError::NotFound(id.to_string()));
        };

        let session = Session::try_from(&row)?;

        Ok(session)
    }
}

#[cfg(test)]
mod tests {
    use crate::database::test_helpers::DbTestBuilder;
    use crate::error::DBError;
    use crate::fixture::fixture_session;

    #[tokio::test]
    async fn test_get_session() {
        let session_id = "session-id-get";
        let session = fixture_session(|s| s.id = session_id.to_string());

        DbTestBuilder::new()
            .with_sessions(vec![session.clone()])
            .run(|db_client| async move {
                let got_session = db_client
                    .get_session(session_id)
                    .await
                    .expect("failed to get session");

                assert_eq!(got_session, session);
            })
            .await;
    }

    #[tokio::test]
    async fn test_get_session_not_found() {
        DbTestBuilder::new()
            .run(|db_client| async move {
                let result = db_client.get_session("non-existent-id").await;
                assert!(matches!(result, Err(DBError::NotFound(_))));
            })
            .await;
    }
}

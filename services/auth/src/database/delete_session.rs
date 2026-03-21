//! Delete session database operation.

use super::client::PostgresDBClient;
use crate::error::DBError;

impl PostgresDBClient {
    /// Deletes a session from the database.
    ///
    /// # Errors
    /// - if the database connection cannot be established
    /// - if the database query fails
    pub(super) async fn delete_session(&self, id: &str) -> Result<(), DBError> {
        let client = self.pool.get().await?;

        client
            .execute("DELETE FROM sessions WHERE id = $1", &[&id])
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::test_helpers::DbTestBuilder;
    use crate::fixture::fixture_session;

    #[tokio::test]
    async fn test_delete_session() {
        let session_id = "session-id-delete";
        let session = fixture_session(|s| s.id = session_id.to_string());

        DbTestBuilder::new()
            .with_sessions(vec![session])
            .run(|db_client| async move {
                db_client
                    .delete_session(session_id)
                    .await
                    .expect("failed to delete session");

                let got_result = db_client.get_session(session_id).await;

                if let Err(DBError::NotFound(s)) = got_result {
                    assert_eq!(s, "session-id-delete");
                } else {
                    panic!("expected NotFound error");
                }
            })
            .await;
    }
}

//! Update session database operation.

use chrono::{DateTime, Utc};

use super::client::PostgresDBClient;
use crate::error::DBError;

impl PostgresDBClient {
    /// Updates a session's expiry in the database.
    ///
    /// # Errors
    /// - if the database connection cannot be established
    /// - if the database query fails
    pub(super) async fn update_session(
        &self,
        id: &str,
        expires_at: &DateTime<Utc>,
    ) -> Result<(), DBError> {
        let client = self.pool.get().await?;

        client
            .execute(
                "UPDATE sessions SET expires_at = $1 WHERE id = $2",
                &[&expires_at, &id],
            )
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::database::test_helpers::DbTestBuilder;
    use crate::fixture::fixture_session;
    use chrono::TimeZone;

    #[tokio::test]
    async fn test_update_session() {
        let session_id = "session-id-update";
        let mut session = fixture_session(|s| s.id = session_id.to_string());

        DbTestBuilder::new()
            .with_sessions(vec![session.clone()])
            .run(|db_client| async move {
                session.expires_at = chrono::Utc.with_ymd_and_hms(2020, 1, 9, 0, 0, 0).unwrap();
                db_client
                    .update_session(session_id, &session.expires_at)
                    .await
                    .expect("failed to update session");

                let got_session = db_client
                    .get_session(session_id)
                    .await
                    .expect("failed to get session");

                assert_eq!(got_session, session);
            })
            .await;
    }
}

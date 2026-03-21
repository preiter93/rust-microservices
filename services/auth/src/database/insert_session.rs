//! Insert session database operation.

use super::client::PostgresDBClient;
use super::model::Session;
use crate::error::DBError;
use setup::session::SESSION_TOKEN_EXPIRY_DURATION;

impl PostgresDBClient {
    /// Inserts a session into the database.
    ///
    /// # Errors
    /// - if the database connection cannot be established
    /// - if the database query fails
    pub(super) async fn insert_session(&self, session: Session) -> Result<(), DBError> {
        let client = self.pool.get().await?;
        let expires_at = session
            .created_at
            .checked_add_signed(SESSION_TOKEN_EXPIRY_DURATION);

        client
            .execute(
                "INSERT INTO sessions (id, secret_hash, user_id, created_at, expires_at) VALUES ($1, $2, $3, $4, $5)",
                &[&session.id, &session.secret_hash, &session.user_id, &session.created_at, &expires_at],
            )
            .await?;

        Ok(())
    }
}

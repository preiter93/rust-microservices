//! Update OAuth account database operation.

use uuid::Uuid;

use super::client::PostgresDBClient;
use super::model::OAuthAccount;
use crate::error::DBError;

impl PostgresDBClient {
    /// Updates the user id of an OAuth account.
    ///
    /// # Errors
    /// - if the database connection cannot be established
    /// - if the row does not exist
    /// - if the database query fails
    pub(super) async fn update_oauth_account(
        &self,
        id: &str,
        user_id: Uuid,
    ) -> Result<OAuthAccount, DBError> {
        let client = self.pool.get().await?;

        let row = client
            .query_opt(
                "UPDATE oauth_accounts
                 SET user_id = $2, updated_at = NOW()
                 WHERE id = $1
                 RETURNING id, provider, external_user_id, external_user_name,
                           external_user_email, access_token, access_token_expires_at,
                           refresh_token, user_id",
                &[&id, &user_id],
            )
            .await?;
        let Some(row) = row else {
            return Err(DBError::NotFound(id.to_string()));
        };

        let oauth_account = OAuthAccount::try_from(&row)?;

        Ok(oauth_account)
    }
}

#[cfg(test)]
mod tests {
    use crate::database::test_helpers::DbTestBuilder;
    use crate::fixture::{fixture_oauth_account, fixture_uuid};

    #[tokio::test]
    async fn test_update_oauth_account() {
        let oauth_id = "oauth-id-update";
        let external_user_id = "external-user-id-update";

        let mut account = fixture_oauth_account(|v| {
            v.id = oauth_id.to_string();
            v.external_user_id = external_user_id.to_string();
        });

        DbTestBuilder::new()
            .with_accounts(vec![account.clone()])
            .run(|db_client| async move {
                let user_id = fixture_uuid();
                account.user_id = Some(user_id);

                let got_account = db_client
                    .update_oauth_account(&oauth_id, user_id)
                    .await
                    .expect("failed to update account");

                assert_eq!(got_account, account);
            })
            .await;
    }
}

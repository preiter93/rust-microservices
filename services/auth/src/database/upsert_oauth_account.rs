//! Upsert OAuth account database operation.

use super::client::PostgresDBClient;
use super::model::OAuthAccount;
use crate::error::DBError;

impl PostgresDBClient {
    /// Inserts or updates an OAuth account. Returns the account after upsert.
    ///
    /// # Errors
    /// - if the database connection cannot be established
    /// - if the database query fails
    pub(super) async fn upsert_oauth_account(
        &self,
        account: &OAuthAccount,
    ) -> Result<OAuthAccount, DBError> {
        let client = self.pool.get().await?;

        let row = client
            .query_one(
                "INSERT INTO oauth_accounts (
                    id, provider, external_user_id, external_user_name,
                    external_user_email, access_token, access_token_expires_at,
                    refresh_token, user_id
                 )
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                 ON CONFLICT (external_user_id) DO UPDATE SET
                    access_token = EXCLUDED.access_token,
                    access_token_expires_at = EXCLUDED.access_token_expires_at,
                    refresh_token = EXCLUDED.refresh_token,
                    updated_at = NOW()
                 RETURNING
                    id, provider, external_user_id, external_user_name,
                    external_user_email, access_token, access_token_expires_at,
                    refresh_token, user_id",
                &[
                    &account.id,
                    &account.provider,
                    &account.external_user_id,
                    &account.external_user_name,
                    &account.external_user_email,
                    &account.access_token,
                    &account.access_token_expires_at,
                    &account.refresh_token,
                    &account.user_id,
                ],
            )
            .await?;

        let oauth_account = OAuthAccount::try_from(&row)?;

        Ok(oauth_account)
    }
}

#[cfg(test)]
mod tests {
    use crate::database::test_helpers::DbTestBuilder;
    use crate::fixture::fixture_oauth_account;

    #[tokio::test]
    async fn test_upsert_oauth_account_insert() {
        let oauth_id = "oauth-id-upsert-insert";
        let external_user_id = "external-user-id-upsert-insert";

        DbTestBuilder::new()
            .run(|db_client| async move {
                let account = fixture_oauth_account(|v| {
                    v.id = oauth_id.to_string();
                    v.external_user_id = external_user_id.to_string();
                });

                let got_account = db_client
                    .upsert_oauth_account(&account)
                    .await
                    .expect("failed to insert account");

                assert_eq!(got_account, account);
            })
            .await;
    }

    #[tokio::test]
    async fn test_upsert_oauth_account_update() {
        let oauth_id = "oauth-id-upsert-update";
        let external_user_id = "external-user-id-upsert-update";

        DbTestBuilder::new()
            .run(|db_client| async move {
                let account = fixture_oauth_account(|v| {
                    v.id = oauth_id.to_string();
                    v.external_user_id = external_user_id.to_string();
                });

                db_client
                    .upsert_oauth_account(&account)
                    .await
                    .expect("failed to insert account");

                let updated_account = fixture_oauth_account(|v| {
                    v.id = oauth_id.to_string();
                    v.external_user_id = external_user_id.to_string();
                    v.access_token = Some(String::from("new-access-token"));
                });

                let got_account = db_client
                    .upsert_oauth_account(&updated_account)
                    .await
                    .expect("failed to upsert account");

                assert_eq!(got_account, updated_account);
            })
            .await;
    }
}

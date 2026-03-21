//! Get OAuth account database operation.

use uuid::Uuid;

use super::client::PostgresDBClient;
use super::model::OAuthAccount;
use crate::error::DBError;
use crate::proto::OauthProvider;

impl PostgresDBClient {
    /// Returns the oauth account from a user id and provider.
    ///
    /// # Errors
    /// - if the database connection cannot be established
    /// - if the row does not exist
    /// - if executing database statement fails
    pub(super) async fn get_oauth_account(
        &self,
        user_id: Uuid,
        provider: OauthProvider,
    ) -> Result<OAuthAccount, DBError> {
        let client = self.pool.get().await?;
        let provider = provider as i32;

        let stmt = client
            .prepare("SELECT id, provider, external_user_id, external_user_name, external_user_email, access_token, access_token_expires_at, refresh_token, user_id FROM oauth_accounts WHERE user_id = $1 AND provider = $2")
            .await?;
        let row = client.query_opt(&stmt, &[&user_id, &provider]).await?;
        let Some(row) = row else {
            return Err(DBError::NotFound(user_id.to_string()));
        };

        Ok(OAuthAccount::try_from(&row)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::test_helpers::DbTestBuilder;
    use crate::error::DBError;
    use crate::fixture::fixture_oauth_account;
    use rstest::rstest;

    #[rstest]
    #[case::happy_path(
        Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
        OauthProvider::Unspecified,
        vec![fixture_oauth_account(|v| {
            v.id = "oauth-id-get".to_string();
            v.external_user_id = "external-user-id-get".to_string();
            v.provider = OauthProvider::Unspecified as i32;
            v.user_id = Some(Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap());
        })],
        Ok(fixture_oauth_account(|v| {
            v.id = "oauth-id-get".to_string();
            v.external_user_id = "external-user-id-get".to_string();
            v.provider = OauthProvider::Unspecified as i32;
            v.user_id = Some(Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap());
        }))
    )]
    #[case::not_found(
        Uuid::parse_str("99999999-9999-9999-9999-999999999999").unwrap(),
        OauthProvider::Unspecified,
        vec![],
        Err(DBError::NotFound("99999999-9999-9999-9999-999999999999".to_string()))
    )]
    #[tokio::test]
    async fn test_get_oauth_account(
        #[case] user_id: Uuid,
        #[case] provider: OauthProvider,
        #[case] given_accounts: Vec<OAuthAccount>,
        #[case] want: Result<OAuthAccount, DBError>,
    ) {
        DbTestBuilder::new()
            .with_accounts(given_accounts)
            .run(|db_client| async move {
                let got = db_client.get_oauth_account(user_id, provider).await;

                match (got, want) {
                    (Ok(got_account), Ok(want_account)) => assert_eq!(got_account, want_account),
                    (Err(got_err), Err(want_err)) => {
                        assert_eq!(format!("{got_err}"), format!("{want_err}"))
                    }
                    (got, want) => panic!("expected {want:?}, got {got:?}"),
                }
            })
            .await;
    }
}

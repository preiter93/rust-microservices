//! Get user database operation.

use uuid::Uuid;

use super::model::User;
use crate::error::DBError;

use super::client::PostgresDBClient;

impl PostgresDBClient {
    /// Gets a user from the database by id.
    ///
    /// # Errors
    /// - if the database connection cannot be established
    /// - if the database query fails
    /// - if the user is not found
    pub(super) async fn get_user(&self, id: Uuid) -> Result<User, DBError> {
        let client = self.pool.get().await?;

        let stmt = client
            .prepare("SELECT id, name, email FROM users WHERE id = $1")
            .await?;
        let row = client.query_opt(&stmt, &[&id]).await?;
        let Some(row) = row else {
            return Err(DBError::NotFound);
        };

        Ok(User::try_from(row)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::test_helpers::DbTestBuilder;
    use crate::fixture::{fixture_user, fixture_uuid};
    use rstest::rstest;

    #[rstest]
    #[case::happy_path(
        fixture_uuid(),
        vec![fixture_user(|_| {})],
        Ok(fixture_user(|_| {}))
    )]
    #[case::not_found(
        Uuid::parse_str("99999999-9999-9999-9999-999999999999").unwrap(),
        vec![],
        Err(DBError::NotFound)
    )]
    #[tokio::test]
    async fn test_get_user(
        #[case] user_id: Uuid,
        #[case] given_users: Vec<User>,
        #[case] want: Result<User, DBError>,
    ) {
        DbTestBuilder::new()
            .with_users(given_users)
            .run(|db_client| async move {
                let got = db_client.get_user(user_id).await;

                match (got, want) {
                    (Ok(got_user), Ok(want_user)) => assert_eq!(got_user, want_user),
                    (Err(got_err), Err(want_err)) => {
                        assert_eq!(format!("{got_err}"), format!("{want_err}"))
                    }
                    (got, want) => panic!("expected {want:?}, got {got:?}"),
                }
            })
            .await;
    }
}

use crate::error::DBError;
use crate::model::User;
use deadpool_postgres::Pool;
use std::fmt::Debug;
use tokio_postgres::Row;
use tonic::async_trait;
use uuid::Uuid;

#[cfg_attr(test, mock::db_client)]
#[async_trait]
pub trait DBClient: Send + Sync + 'static {
    async fn insert_user(&self, user: &User) -> Result<(), DBError>;

    async fn get_user(&self, id: Uuid) -> Result<User, DBError>;
}

#[derive(Clone, Debug)]
pub struct PostgresDBClient {
    pub pool: Pool,
}

impl PostgresDBClient {
    #[must_use]
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DBClient for PostgresDBClient {
    /// # Errors
    /// - if the database connection cannot be established
    /// - if the database query fails
    async fn insert_user(&self, user: &User) -> Result<(), DBError> {
        let client = self.pool.get().await?;

        client
            .execute(
                "INSERT INTO users (id, name, email) VALUES ($1, $2, $3)",
                &[&user.id, &user.name, &user.email],
            )
            .await?;

        Ok(())
    }

    /// # Errors
    /// - if the database connection cannot be established
    /// - if the database query fails
    /// - If the user is not found
    async fn get_user(&self, id: Uuid) -> Result<User, DBError> {
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

impl TryFrom<Row> for User {
    type Error = DBError;

    fn try_from(row: Row) -> Result<Self, DBError> {
        let id: Uuid = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let email: String = row.try_get("email")?;

        Ok(User { id, name, email })
    }
}

#[cfg(test)]
pub mod test {
    pub(crate) use super::MockDBClient;
    use super::*;
    use crate::fixture::{fixture_user, fixture_uuid};
    use crate::model::User;
    use rstest::rstest;
    use testutils::get_test_db;
    use user::SERVICE_NAME;
    use uuid::Uuid;

    async fn run_db_test<F, Fut>(given_users: Vec<User>, test_fn: F)
    where
        F: FnOnce(PostgresDBClient) -> Fut,
        Fut: std::future::Future<Output = ()>,
    {
        let migrations = std::fs::canonicalize("./migrations").unwrap();
        let pool = get_test_db(SERVICE_NAME, migrations)
            .await
            .expect("failed to get connection to test db");
        let db_client = PostgresDBClient { pool };

        for user in &given_users {
            db_client
                .insert_user(user)
                .await
                .expect("failed to insert user");
        }

        test_fn(db_client).await;
    }

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
        run_db_test(given_users, |db_client| async move {
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

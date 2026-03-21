use chrono::{DateTime, Utc};
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Session {
    pub id: String,
    pub secret_hash: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub user_id: Uuid,
}

impl TryFrom<&Row> for Session {
    type Error = tokio_postgres::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(Session {
            id: row.try_get("id")?,
            secret_hash: row.try_get("secret_hash")?,
            created_at: row.try_get("created_at")?,
            expires_at: row.try_get("expires_at")?,
            user_id: row.try_get("user_id")?,
        })
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct OAuthAccount {
    pub id: String,
    pub provider: i32,
    pub external_user_id: String,
    pub external_user_name: Option<String>,
    pub external_user_email: Option<String>,
    pub access_token: Option<String>,
    pub access_token_expires_at: Option<DateTime<Utc>>,
    pub refresh_token: Option<String>,
    pub user_id: Option<Uuid>,
}

impl TryFrom<&Row> for OAuthAccount {
    type Error = tokio_postgres::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        Ok(OAuthAccount {
            id: row.try_get("id")?,
            provider: row.try_get("provider")?,
            external_user_id: row.try_get("external_user_id")?,
            external_user_name: row.try_get("external_user_name")?,
            external_user_email: row.try_get("external_user_email")?,
            access_token: row.try_get("access_token")?,
            access_token_expires_at: row.try_get("access_token_expires_at")?,
            refresh_token: row.try_get("refresh_token")?,
            user_id: row.try_get("user_id")?,
        })
    }
}

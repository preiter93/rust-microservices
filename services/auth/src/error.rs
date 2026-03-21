use derive_tonic_status::Status;
use thiserror::Error;

#[derive(Debug, Error, Status)]
#[non_exhaustive]
pub enum Error {
    #[error("missing user id")]
    #[status(InvalidArgument)]
    MissingUserId,

    #[error("invalid user id: {0}")]
    #[status(InvalidArgument)]
    InvalidUserId(String),

    #[error("missing oauth account id")]
    #[status(InvalidArgument)]
    MissingOauthAccountID,

    #[error("missing token")]
    #[status(Unauthenticated)]
    MissingToken,

    #[error("invalid token")]
    #[status(Unauthenticated)]
    InvalidToken,

    #[error("token expired")]
    #[status(Unauthenticated)]
    ExpiredToken,

    #[error("token secret mismatch")]
    #[status(Unauthenticated)]
    SecretMismatch,

    #[error("token not found")]
    #[status(Unauthenticated)]
    NotFound,

    #[error("get session error: {0}")]
    #[status(Internal)]
    GetSession(DBError),

    #[error("delete session error: {0}")]
    #[status(Internal)]
    DeleteSession(DBError),

    #[error("insert session error: {0}")]
    #[status(Internal)]
    InsertSession(DBError),

    #[error("update oauth account error: {0}")]
    #[status(Internal)]
    UpdateOauthAccount(DBError),

    #[error("get oauth account error: {0}")]
    #[status(Internal)]
    GetOauthAccount(DBError),

    #[error("oauth provider is not specified")]
    #[status(InvalidArgument)]
    UnspecifiedOauthProvider,

    #[error("upsert oauth account error: {0}")]
    #[status(Internal)]
    UpsertOauthAccount(DBError),
}

// Database error
#[derive(Debug, Error)]
pub enum DBError {
    #[error("unknown error occured")]
    Unknown,

    #[error("internal database error: {0}")]
    Internal(#[from] tokio_postgres::Error),

    #[error("connection error: {0}")]
    Connection(#[from] deadpool_postgres::PoolError),

    #[error("entity not found: {0}")]
    NotFound(String),
}

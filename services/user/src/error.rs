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

    #[error("missing user name")]
    #[status(InvalidArgument)]
    MissingUserName,

    #[error("missing user email")]
    #[status(InvalidArgument)]
    MissingUserEmail,

    #[error("user not found: {0}")]
    #[status(NotFound)]
    UserNotFound(String),

    #[error("get user error: {0}")]
    #[status(Internal)]
    GetUser(DBError),

    #[error("insert user error: {0}")]
    #[status(Internal)]
    InsertUser(DBError),
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

    #[error("entity not found")]
    NotFound,
}

use derive_tonic_status::Status;
use thiserror::Error;

/// Error for [`crate::proto::api_service_server::ApiService::create_entity`]
#[derive(Debug, Error, Status)]
#[non_exhaustive]
pub enum Error {
    #[error("missing entity id")]
    #[status(InvalidArgument)]
    MissingEntityId,

    #[error("invalid entity id: {0}")]
    #[status(InvalidArgument)]
    InvalidEntityId(String),

    #[error("entity not found: {0}")]
    #[status(NotFound)]
    EntityNotFound(String),

    #[error("get entity error: {0}")]
    #[status(Internal)]
    GetEntity(DBError),
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

    #[error("Entity not found")]
    NotFound,
}

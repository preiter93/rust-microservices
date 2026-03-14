use derive_tonic_status::Status;
use thiserror::Error;

#[derive(Debug, Error, Status)]
#[non_exhaustive]
pub enum Error {
    #[error("oauth error: {0}")]
    #[status(Internal)]
    Error(#[from] oauth::Error),

    #[error("missing id token")]
    #[status(Internal)]
    MissingIDToken,

    #[error("missing kid in token")]
    #[status(Internal)]
    MissingKID,

    #[error("no matchin jwks found")]
    #[status(Internal)]
    NoMatchingJWKS,

    #[error("missing access token")]
    #[status(Internal)]
    MissingAccessToken,

    #[error("missing expires in")]
    #[status(Internal)]
    MissingExpiresIn,

    #[error("missing x user id")]
    #[status(Internal)]
    MissingXUserID,

    #[error("missing email")]
    #[status(Internal)]
    MissingEmail,

    #[error("reqwest error: {0}")]
    #[status(Internal)]
    Reqwest(#[from] reqwest::Error),

    #[error("unexpected HTTP status code: {0}")]
    #[status(Internal)]
    UnexpectedStatusCode(reqwest::StatusCode),
}

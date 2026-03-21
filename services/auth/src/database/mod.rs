//! Database module for the auth service.

mod client;
mod delete_session;
mod get_oauth_account;
mod get_session;
mod insert_session;
mod model;
mod update_oauth_account;
mod update_session;
mod upsert_oauth_account;

#[cfg(test)]
mod test_helpers;

pub use client::{DBClient, PostgresDBClient};
pub use model::{OAuthAccount, Session};

#[cfg(test)]
pub(crate) use client::MockDBClient;

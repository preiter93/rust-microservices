//! Database module for the user service.

mod client;
mod get_user;
mod insert_user;
mod model;

#[cfg(test)]
mod test_helpers;

pub use client::{DBClient, PostgresDBClient};
pub use model::User;

#[cfg(test)]
pub(crate) use client::MockDBClient;

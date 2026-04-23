//! Database module for the dummy service.

mod client;
mod get_entity;
mod insert_entity;
mod model;

#[cfg(test)]
mod test_helpers;

// Re-exports
pub use client::{DBClient, PostgresDBClient};
pub use model::Entity;

#[cfg(test)]
pub(crate) use client::MockDBClient;

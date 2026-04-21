pub mod create_user;
pub mod database;
pub mod error;
pub mod get_user;
pub mod server;

#[cfg(test)]
mod fixtures;

use crate::server::Server;
use api::user_service_server::UserServiceServer;
use api::{GRPC_PORT, SERVICE_NAME};
use common::UuidV4Generator;
use database::PostgresDBClient;
use dotenv::dotenv;
use setup::{middleware::TracingGrpcServiceLayer, tracing::init_tracer};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let tracer = init_tracer(SERVICE_NAME)?;

    let pg_cfg = ::database::PGConfig::from_env(SERVICE_NAME)?;
    let pool = ::database::connect(&pg_cfg)?;
    ::database::run_migrations!(pool, "./migrations");

    let server = Server {
        db: PostgresDBClient::new(pool),
        uuid: UuidV4Generator,
    };

    let addr = format!("0.0.0.0:{GRPC_PORT}").parse()?;
    let svc = UserServiceServer::new(server);

    println!("listening on :{GRPC_PORT}");
    let mut server = tonic::transport::Server::builder().layer(TracingGrpcServiceLayer);
    server.add_service(svc).serve(addr).await.unwrap();

    tracer.shutdown()?;

    Ok(())
}

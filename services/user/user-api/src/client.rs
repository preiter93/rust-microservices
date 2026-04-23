// This file is generated.
use crate::GRPC_PORT;
use crate::SERVICE_NAME;
use crate::proto::CreateUserReq;
use crate::proto::CreateUserResp;
use crate::proto::GetUserReq;
use crate::proto::GetUserResp;
use crate::proto::user_service_client::UserServiceClient;
use setup::{middleware::tracing::TracingServiceClient, patched_host};
use std::{error::Error, str::FromStr as _};
use tonic::transport::{Channel, Endpoint};
use tonic::{Request, Response, Status, async_trait};

#[derive(Clone)]
pub struct UserApiClient(UserServiceClient<TracingServiceClient<Channel>>);

impl UserApiClient {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let host = patched_host(String::from(SERVICE_NAME));
        let endpoint = Endpoint::from_str(&format!("http://{host}:{GRPC_PORT}"))?;
        let channel = endpoint.connect().await?;
        let client = TracingServiceClient::new(channel);
        let client = UserServiceClient::new(client);

        Ok(Self(client))
    }
}

#[rustfmt::skip]
#[async_trait]
pub trait IUserApiClient: Send + Sync + 'static {
    async fn create_user(&self, req: Request<CreateUserReq>) -> Result<Response<CreateUserResp>, Status>;
    async fn get_user(&self, req: Request<GetUserReq>) -> Result<Response<GetUserResp>, Status>;
}

#[rustfmt::skip]
#[async_trait]
impl IUserApiClient for UserApiClient {
    async fn create_user(&self, req: Request<CreateUserReq>) -> Result<Response<CreateUserResp>, Status> {
        self.0.clone().create_user(req).await
    }
    async fn get_user(&self, req: Request<GetUserReq>) -> Result<Response<GetUserResp>, Status> {
        self.0.clone().get_user(req).await
    }
}

#[cfg(feature = "testutils")]
pub mod testutils {
    use super::*;
    use tokio::sync::Mutex;
    use tonic::{Request, Response, Status};

    #[rustfmt::skip]
    pub struct MockUserApiClient {
        pub create_user_req: Mutex<Option<CreateUserReq>>,
        pub create_user_resp: Mutex<Option<Result<CreateUserResp, Status>>>,
        pub get_user_req: Mutex<Option<GetUserReq>>,
        pub get_user_resp: Mutex<Option<Result<GetUserResp, Status>>>,
    }

    impl Default for MockUserApiClient {
        fn default() -> Self {
            Self {
                create_user_req: Mutex::new(None),
                create_user_resp: Mutex::new(None),
                get_user_req: Mutex::new(None),
                get_user_resp: Mutex::new(None),
            }
        }
    }

    #[rustfmt::skip]
    #[async_trait]
    impl IUserApiClient for MockUserApiClient {
        async fn create_user(&self, req: Request<CreateUserReq>) -> Result<Response<CreateUserResp>, Status> {
            *self.create_user_req.lock().await = Some(req.into_inner());
            self.create_user_resp.lock().await.take().unwrap().map(Response::new)
        }
        async fn get_user(&self, req: Request<GetUserReq>) -> Result<Response<GetUserResp>, Status> {
            *self.get_user_req.lock().await = Some(req.into_inner());
            self.get_user_resp.lock().await.take().unwrap().map(Response::new)
        }
    }
}

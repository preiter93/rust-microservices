use crate::{
    database::{DBClient, User},
    error::Error,
    handler::Handler,
    proto::{CreateUserReq, CreateUserResp},
};
use common::UuidGenerator;
use tonic::{Request, Response, Status};

impl<D, U> Handler<D, U>
where
    D: DBClient,
    U: UuidGenerator,
{
    /// Creates a new user.
    ///
    /// # Errors
    /// - `InvalidArgument` if the user is missing, name is empty, or email is empty
    /// - `Internal` if the user cannot be inserted into the db
    pub async fn create_user(
        &self,
        req: Request<CreateUserReq>,
    ) -> Result<Response<CreateUserResp>, Status> {
        let req = req.into_inner();

        let new_user = req.user.ok_or(Error::MissingUser)?;

        let user = User::from_proto(self.uuid.generate(), new_user)?;

        tracing::Span::current().record("user_id", user.id.to_string());

        self.db
            .insert_user(&user)
            .await
            .map_err(Error::InsertUser)?;

        let response = CreateUserResp {
            user: Some(user.to_proto()),
        };

        Ok(Response::new(response))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        database::MockDBClient,
        error::DBError,
        fixture::{fixture_create_user_req, fixture_proto_user},
        handler::Handler,
        proto::CreateUserReq,
        proto::CreateUserResp,
    };
    use common::mock::MockUuidGenerator;
    use rstest::rstest;
    use tokio::sync::Mutex;
    use tonic::{Code, Request};

    #[rstest]
    #[case::happy(
        fixture_create_user_req(|_| {}),
        Ok(()),
        Ok(CreateUserResp { user: Some(fixture_proto_user(|_| {})) })
    )]
    #[case::missing_user(
        fixture_create_user_req(|r| r.user = None),
        Ok(()),
        Err(Code::InvalidArgument)
    )]
    #[case::missing_name(
        fixture_create_user_req(|r| r.user.as_mut().unwrap().name.clear()),
        Ok(()),
        Err(Code::InvalidArgument)
    )]
    #[case::missing_email(
        fixture_create_user_req(|r| r.user.as_mut().unwrap().email.clear()),
        Ok(()),
        Err(Code::InvalidArgument)
    )]
    #[case::internal_error(
        fixture_create_user_req(|_| {}),
        Err(DBError::Unknown),
        Err(Code::Internal)
    )]
    #[tokio::test]
    async fn test_create_user(
        #[case] req: CreateUserReq,
        #[case] insert_res: Result<(), DBError>,
        #[case] want: Result<CreateUserResp, Code>,
    ) {
        use testutils::assert_response;

        let db = MockDBClient {
            insert_user: Mutex::new(Some(insert_res)),
            ..Default::default()
        };

        let service = Handler {
            db,
            uuid: MockUuidGenerator::default(),
        };

        let got = service.create_user(Request::new(req)).await;
        assert_response(got, want);
    }
}

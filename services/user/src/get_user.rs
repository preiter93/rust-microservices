use crate::{
    db::DBClient,
    error::{DBError, Error},
    handler::Handler,
    proto::{GetUserReq, GetUserResp},
};
use common::UuidGenerator;
use setup::validate_user_id;
use tonic::{Request, Response, Status};

impl<D, U> Handler<D, U>
where
    D: DBClient,
    U: UuidGenerator,
{
    /// Gets a user by identifier.
    ///
    /// # Errors
    /// - `InvalidArgument` if the user id is empty or invalid
    /// - `NotFound` if the user is not found
    /// - `Internal` if the database query fails
    pub async fn get_user(
        &self,
        req: Request<GetUserReq>,
    ) -> Result<Response<GetUserResp>, Status> {
        let req = req.into_inner();
        let user_id = validate_user_id(&req.id)?;

        let user = self.db.get_user(user_id).await.map_err(|e| match e {
            DBError::NotFound => Error::UserNotFound(user_id.to_string()),
            _ => Error::GetUser(e),
        })?;

        Ok(Response::new(GetUserResp {
            user: Some(user.into()),
        }))
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use tokio::sync::Mutex;
    use tonic::{Code, Request};

    use crate::{
        db::test::MockDBClient,
        error::DBError,
        fixture::{fixture_get_user_req, fixture_get_user_resp, fixture_user},
        handler::Handler,
        model::User,
        proto::{GetUserReq, GetUserResp},
    };

    #[rstest]
    #[case::happy_path(
        fixture_get_user_req(|_| {}),
        Ok(fixture_user(|_| {})),
        Ok(fixture_get_user_resp(|_| {})),
    )]
    #[case::missing_id(
        fixture_get_user_req(|r| { r.id = "".to_string() }),
        Ok(fixture_user(|_| {})),
        Err(Code::InvalidArgument)
    )]
    #[case::not_a_uuid(
        fixture_get_user_req(|r| { r.id = "not-a-uuid".to_string() }),
        Ok(fixture_user(|_| {})),
        Err(Code::InvalidArgument)
    )]
    #[case::not_found(
        fixture_get_user_req(|_| {}),
        Err(DBError::NotFound),
        Err(Code::NotFound)
    )]
    #[case::internal_error(
        fixture_get_user_req(|_| {}),
        Err(DBError::Unknown),
        Err(Code::Internal)
    )]
    #[tokio::test]
    async fn test_get_user(
        #[case] req: GetUserReq,
        #[case] db_result: Result<User, DBError>,
        #[case] want: Result<GetUserResp, Code>,
    ) {
        // given
        use common::mock::MockUuidGenerator;
        use testutils::assert_response;
        let db = MockDBClient {
            get_user: Mutex::new(Some(db_result)),
            ..Default::default()
        };
        let service = Handler {
            db,
            uuid: MockUuidGenerator::default(),
        };

        // when
        let got = service.get_user(Request::new(req)).await;

        // then
        assert_response(got, want);
    }
}

use crate::{
    db::DBClient,
    error::Error,
    handler::Handler,
    proto::{CreateEntityReq, CreateEntityResp, Entity},
};
use common::UuidGenerator;
use setup::validate_user_id;
use tonic::{Request, Response, Status};

impl<D, U> Handler<D, U>
where
    D: DBClient,
    U: UuidGenerator,
{
    /// Creates a new entity.
    ///
    /// # Errors
    /// - `InvalidArgument` if the user id is empty or invalid
    /// - `Internal` if the database insert fails
    pub async fn create_entity(
        &self,
        req: Request<CreateEntityReq>,
    ) -> Result<Response<CreateEntityResp>, Status> {
        let req = req.into_inner();

        let user_id = validate_user_id(&req.user_id)?;

        let id = self.uuid.generate();

        self.db
            .insert_entity(id, user_id)
            .await
            .map_err(Error::InsertEntity)?;

        let response = CreateEntityResp {
            entity: Some(Entity { id: id.to_string() }),
        };

        Ok(Response::new(response))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        db::test::MockDBClient,
        error::DBError,
        fixture::{fixture_create_entity_req, fixture_entity},
        handler::Handler,
        proto::{CreateEntityReq, CreateEntityResp},
    };
    use common::mock::MockUuidGenerator;
    use rstest::rstest;
    use tokio::sync::Mutex;
    use tonic::{Code, Request};

    #[rstest]
    #[case::happy_path(
        fixture_create_entity_req(|_| {}),
        Ok(()),
        Ok(CreateEntityResp { entity: Some(fixture_entity(|_| {})) })
    )]
    #[case::missing_user_id(
        fixture_create_entity_req(|r| r.user_id.clear()),
        Ok(()),
        Err(Code::InvalidArgument)
    )]
    #[case::internal_error(
        fixture_create_entity_req(|_| {}),
        Err(DBError::Unknown),
        Err(Code::Internal)
    )]
    #[tokio::test]
    async fn test_create_entity(
        #[case] req: CreateEntityReq,
        #[case] db_result: Result<(), DBError>,
        #[case] want: Result<CreateEntityResp, Code>,
    ) {
        use testutils::assert_response;

        let db = MockDBClient {
            insert_entity: Mutex::new(Some(db_result)),
            ..Default::default()
        };

        let service = Handler {
            db,
            uuid: MockUuidGenerator::default(),
        };

        let got = service.create_entity(Request::new(req)).await;
        assert_response(got, want);
    }
}

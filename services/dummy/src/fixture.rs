#![cfg(test)]

use uuid::Uuid;

use crate::model::Entity;
use crate::proto::{self, CreateEntityReq, GetEntityReq, GetEntityResp, NewEntity};

pub fn fixture_uuid() -> Uuid {
    Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap()
}

pub fn fixture_entity<F>(mut func: F) -> Entity
where
    F: FnMut(&mut Entity),
{
    let mut entity = Entity {
        id: fixture_uuid(),
        user_id: fixture_uuid(),
        name: "Test Entity".to_string(),
    };
    func(&mut entity);
    entity
}

pub fn fixture_proto_entity<F>(mut func: F) -> proto::Entity
where
    F: FnMut(&mut proto::Entity),
{
    let mut entity = proto::Entity {
        id: fixture_uuid().to_string(),
        name: "Test Entity".to_string(),
    };
    func(&mut entity);
    entity
}

pub fn fixture_new_entity<F>(mut func: F) -> NewEntity
where
    F: FnMut(&mut NewEntity),
{
    let mut entity = NewEntity {
        name: "Test Entity".to_string(),
    };
    func(&mut entity);
    entity
}

pub fn fixture_create_entity_req<F>(mut func: F) -> CreateEntityReq
where
    F: FnMut(&mut CreateEntityReq),
{
    let mut req = CreateEntityReq {
        user_id: fixture_uuid().to_string(),
        entity: Some(fixture_new_entity(|_| {})),
    };
    func(&mut req);
    req
}

pub fn fixture_get_entity_req<F>(mut func: F) -> GetEntityReq
where
    F: FnMut(&mut GetEntityReq),
{
    let mut req = GetEntityReq {
        id: fixture_uuid().to_string(),
        user_id: fixture_uuid().to_string(),
    };
    func(&mut req);
    req
}

pub fn fixture_get_entity_resp<F>(mut func: F) -> GetEntityResp
where
    F: FnMut(&mut GetEntityResp),
{
    let mut resp = GetEntityResp {
        entity: Some(fixture_proto_entity(|_| {})),
    };
    func(&mut resp);
    resp
}

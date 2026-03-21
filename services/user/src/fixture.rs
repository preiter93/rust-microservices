#![cfg(test)]

use uuid::Uuid;

use crate::database::User;
use crate::proto::{self, CreateUserReq, GetUserReq, GetUserResp, NewUser};

pub fn fixture_uuid() -> Uuid {
    Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap()
}

pub fn fixture_user<F>(mut func: F) -> User
where
    F: FnMut(&mut User),
{
    let mut user = User {
        id: fixture_uuid(),
        name: "name".to_string(),
        email: "email".to_string(),
    };
    func(&mut user);
    user
}

pub fn fixture_proto_user<F>(mut func: F) -> proto::User
where
    F: FnMut(&mut proto::User),
{
    let mut user = proto::User {
        id: fixture_uuid().to_string(),
        name: "name".to_string(),
        email: "email".to_string(),
    };
    func(&mut user);
    user
}

pub fn fixture_new_user<F>(mut func: F) -> NewUser
where
    F: FnMut(&mut NewUser),
{
    let mut user = NewUser {
        name: "name".to_string(),
        email: "email".to_string(),
    };
    func(&mut user);
    user
}

pub fn fixture_create_user_req<F>(mut func: F) -> CreateUserReq
where
    F: FnMut(&mut CreateUserReq),
{
    let mut req = CreateUserReq {
        user: Some(fixture_new_user(|_| {})),
    };
    func(&mut req);
    req
}

pub fn fixture_get_user_req<F>(mut func: F) -> GetUserReq
where
    F: FnMut(&mut GetUserReq),
{
    let mut req = GetUserReq {
        id: fixture_uuid().to_string(),
    };
    func(&mut req);
    req
}

pub fn fixture_get_user_resp<F>(mut func: F) -> GetUserResp
where
    F: FnMut(&mut GetUserResp),
{
    let mut resp = GetUserResp {
        user: Some(fixture_proto_user(|_| {})),
    };
    func(&mut resp);
    resp
}

use uuid::Uuid;

use crate::error::Error;
use crate::proto;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl User {
    /// Creates a new user from a proto NewUser.
    ///
    /// # Errors
    /// - `InvalidArgument` if the name is empty
    /// - `InvalidArgument` if the email is empty
    pub fn new(id: Uuid, new_user: proto::NewUser) -> Result<Self, Error> {
        if new_user.name.is_empty() {
            return Err(Error::MissingUserName);
        }

        if new_user.email.is_empty() {
            return Err(Error::MissingUserEmail);
        }

        Ok(Self {
            id,
            name: new_user.name,
            email: new_user.email,
        })
    }
}

impl From<User> for proto::User {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            name: user.name,
            email: user.email,
        }
    }
}

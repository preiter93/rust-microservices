//! Database model definitions.

use tokio_postgres::Row;
use uuid::Uuid;

use crate::error::{DBError, Error};
use crate::proto;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

impl User {
    /// Creates a user from a proto NewUser.
    ///
    /// # Errors
    /// - `InvalidArgument` if the name is empty
    /// - `InvalidArgument` if the email is empty
    pub fn from_proto(id: Uuid, new_user: proto::NewUser) -> Result<Self, Error> {
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

    /// Converts the user to a proto User.
    pub fn to_proto(self) -> proto::User {
        proto::User {
            id: self.id.to_string(),
            name: self.name,
            email: self.email,
        }
    }
}

impl TryFrom<Row> for User {
    type Error = DBError;

    fn try_from(row: Row) -> Result<Self, DBError> {
        let id: Uuid = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let email: String = row.try_get("email")?;

        Ok(User { id, name, email })
    }
}

//! Database model definitions.

use tokio_postgres::Row;
use uuid::Uuid;

use crate::error::{DBError, Error};
use crate::proto;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
}

impl Entity {
    /// Creates an entity from a proto NewEntity.
    ///
    /// # Errors
    /// - `InvalidArgument` if the name is empty
    pub fn from_proto(
        id: Uuid,
        user_id: Uuid,
        new_entity: proto::NewEntity,
    ) -> Result<Self, Error> {
        if new_entity.name.is_empty() {
            return Err(Error::MissingEntityName);
        }

        Ok(Self {
            id,
            user_id,
            name: new_entity.name,
        })
    }

    /// Converts the entity to a proto Entity.
    pub fn to_proto(self) -> proto::Entity {
        proto::Entity {
            id: self.id.to_string(),
            name: self.name,
        }
    }
}

impl TryFrom<Row> for Entity {
    type Error = DBError;

    fn try_from(row: Row) -> Result<Self, DBError> {
        let id: Uuid = row.try_get("id")?;
        let user_id: Uuid = row.try_get("user_id")?;
        let name: String = row.try_get("name")?;

        Ok(Entity { id, user_id, name })
    }
}

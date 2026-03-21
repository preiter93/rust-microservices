use uuid::Uuid;

use crate::error::Error;
use crate::proto;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
}

impl Entity {
    /// Creates a new entity from a proto NewEntity.
    ///
    /// # Errors
    /// - `InvalidArgument` if the name is empty
    pub fn new(id: Uuid, user_id: Uuid, new_entity: proto::NewEntity) -> Result<Self, Error> {
        if new_entity.name.is_empty() {
            return Err(Error::MissingEntityName);
        }

        Ok(Self {
            id,
            user_id,
            name: new_entity.name,
        })
    }
}

impl From<Entity> for proto::Entity {
    fn from(entity: Entity) -> Self {
        Self {
            id: entity.id.to_string(),
            name: entity.name,
        }
    }
}

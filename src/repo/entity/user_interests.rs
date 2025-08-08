use crate::repo::entity::error::BuilderError;

pub struct Entity {
    pub user_id: uuid::Uuid,
    pub interests: Vec<uuid::Uuid>,
}

pub struct Builder {
    user_id: Option<uuid::Uuid>,
    interests: Vec<uuid::Uuid>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            user_id: None,
            interests: Vec::new(),
        }
    }

    pub fn build(&mut self) -> Result<Entity, BuilderError> {
        let user_id = self
            .user_id
            .take()
            .ok_or(BuilderError::NotEnoughElement("no user_id".to_string()))?;

        Ok(Entity {
            user_id,
            interests: self.interests.clone(),
        })
    }

    pub fn add_user_id(&mut self, user_id: uuid::Uuid) -> &mut Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn add_interest_id(&mut self, interest_id: uuid::Uuid) -> &mut Self {
        self.interests.push(interest_id);
        self
    }
}

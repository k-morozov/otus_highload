// use argon2::Argon2;
// use argon2::password_hash::{PasswordHasher, SaltString};
// use b64::{STANDARD, ToBase64};

use crate::repo::entity::error::BuilderError;

pub struct Entity {
    pub user_id: uuid::Uuid,
    pub password: String,
}

pub struct Builder {
    user_id: Option<uuid::Uuid>,
    password: Option<String>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            user_id: None,
            password: None,
        }
    }

    pub fn build(&mut self) -> Result<Entity, BuilderError> {
        let user_id = self
            .user_id
            .take()
            .ok_or_else(|| BuilderError::NotEnoughElement("no user_id".to_string()))?;
        let password = self
            .password
            .take()
            .ok_or_else(|| BuilderError::NotEnoughElement("no password".to_string()))?;

        Ok(Entity { user_id, password })
    }

    pub fn add_user_id(&mut self, user_id: uuid::Uuid) -> &mut Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn add_password(&mut self, password: &str) -> &mut Self {
        self.password = Some(String::from(password));
        self
    }
}

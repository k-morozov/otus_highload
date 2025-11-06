use crate::model::user_register;
use crate::repo::entity::error::BuilderError;

pub struct Entity {
    pub user_id: uuid::Uuid,
    pub login: Option<String>,
    pub name: String,
    pub surname: String,
    pub birth_date: Option<String>,
    pub gender: Option<String>,
    pub city_id: Option<uuid::Uuid>,
}

pub struct Builder {
    model: Option<user_register::UserRegisterRequestBody>,
    user_id: Option<uuid::Uuid>,
    city_id: Option<uuid::Uuid>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            model: None,
            user_id: None,
            city_id: None,
        }
    }

    pub fn build(&mut self) -> Result<Entity, BuilderError> {
        let model = self
            .model
            .take()
            .ok_or(BuilderError::NotEnoughElement("no model".to_string()))?;
        let user_id = self
            .user_id
            .take()
            .ok_or(BuilderError::NotEnoughElement("no user_id".to_string()))?;
        let city_id = self
            .city_id
            .take()
            .ok_or(BuilderError::NotEnoughElement("no city_id".to_string()))?;

        Ok(Entity {
            user_id,
            login: Some(model.login),
            name: model.name,
            surname: model.surname,
            birth_date: Some(model.birth_date),
            gender: Some(model.gender),
            city_id: Some(city_id),
        })
    }

    pub fn add_model(&mut self, model: user_register::UserRegisterRequestBody) -> &mut Self {
        self.model = Some(model);
        self
    }

    pub fn add_user_id(&mut self, user_id: uuid::Uuid) -> &mut Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn add_city_id(&mut self, city_id: uuid::Uuid) -> &mut Self {
        self.city_id = Some(city_id);
        self
    }
}

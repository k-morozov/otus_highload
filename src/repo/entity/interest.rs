use super::error::BuilderError;

pub struct Entity {
    pub interests: Vec<(uuid::Uuid, String)>,
}

pub struct Builder {
    interests: Vec<(uuid::Uuid, String)>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            interests: Vec::new(),
        }
    }

    pub fn build(&mut self) -> Result<Entity, BuilderError> {
        if self.interests.is_empty() {
            return Err(BuilderError::NotEnoughElement("no interests".to_string()));
        }

        Ok(Entity {
            interests: self.interests.clone(),
        })
    }

    pub fn add_interes(&mut self, interest_id: uuid::Uuid, interest_name: String) -> &mut Self {
        self.interests.push((interest_id, interest_name));
        self
    }
}

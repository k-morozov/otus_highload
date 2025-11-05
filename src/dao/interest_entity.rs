use super::error::BuilderError;

pub struct InterestEntity {
    pub interests: Vec<(uuid::Uuid, String)>,
}

pub struct Builder {
    // hash?
    interests: Vec<(uuid::Uuid, String)>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            interests: Vec::new(),
        }
    }

    pub fn build(&mut self) -> Result<InterestEntity, BuilderError> {
        if self.interests.is_empty() {
            return Err(BuilderError::NotEnoughElement("no interests".to_string()));
        }

        Ok(InterestEntity {
            interests: self.interests.clone(),
        })
    }

    pub fn add_interest(&mut self, interest_id: uuid::Uuid, interest_name: String) -> &mut Self {
        self.interests.push((interest_id, interest_name));
        self
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRegisterRequestBody {
    pub login: String,
    pub password: String,
    pub name: String,
    pub surname: String,
    pub birth_date: String,
    pub gender: String,
    pub interests: Vec<String>,
    pub city: String,
}

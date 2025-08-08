use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRegisterRequestBody {
    pub name: String,
    pub surname: String,
    pub birth_date: String,
    gender: u8,
    interests: Vec<String>,
    city: String,
}

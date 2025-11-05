use crate::model::user_register;

pub struct CityEntity {
    pub city_name: String,
}

impl From<user_register::UserRegisterRequestBody> for CityEntity {
    fn from(model: user_register::UserRegisterRequestBody) -> Self {
        CityEntity {
            city_name: model.city,
        }
    }
}
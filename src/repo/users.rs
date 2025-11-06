use async_trait::async_trait;
use sqlx::{Execute, Row};
use uuid::Uuid;

use super::users_aux::UsersAux;
use crate::repo::entity::user;
use crate::repo::repository::Repository;
use crate::store::DatabaseResult;
use crate::store::error::StoreError;

pub struct Users {}

impl Users {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Repository<Uuid, user::Entity> for Users {
    type TDatabase = sqlx::Postgres;

    async fn create<'a, E: sqlx::Executor<'a, Database = Self::TDatabase>>(
        &self,
        e: E,
        entity: &user::Entity,
    ) -> DatabaseResult<()> {
        let raw_query = "
            INSERT INTO users 
            (
                user_id,
                login,
                name,
                surname,
                birth_date,
                gender,
                city_id
            ) 
            VALUES ($1, $2, $3, $4, $5::date, $6, $7);
        ";

        let query = sqlx::query(raw_query)
            .bind(entity.user_id)
            .bind(&entity.login)
            .bind(&entity.name)
            .bind(&entity.surname)
            .bind(&entity.birth_date)
            .bind(&entity.gender)
            .bind(entity.city_id);

        tracing::info!("user repo call create with query={}", query.sql());

        query
            .execute(e)
            .await
            .map_err(|e| StoreError::ExecutionFailed(e.to_string()))?;

        return Ok(());
    }

    async fn get_by_id<'a, E: sqlx::Executor<'a, Database = Self::TDatabase>>(
        &self,
        e: E,
        user_id: &Uuid,
    ) -> DatabaseResult<Option<user::Entity>> {
        let raw_query = "
            SELECT name, surname FROM users WHERE user_id=$1;
        ";

        let query = sqlx::query(raw_query).bind(user_id);

        tracing::info!("user repo call get_by_id with query={}", query.sql());

        let result_query = query
            .fetch_optional(e)
            .await
            .map_err(|e| StoreError::ExecutionFailed(e.to_string()))?;

        let row = result_query
            .ok_or_else(|| StoreError::NoData(String::from("no user with the user_id")))?;

        tracing::debug!("got the data about user_id {row:?}");

        if row.is_empty() {
            return Err(StoreError::NoData(String::from("no user")));
        }

        let name: String = row.get(0);
        let surname: String = row.get(1);

        let r = user::Entity {
            user_id: *user_id,
            name,
            surname,
            login: None,
            birth_date: None,
            gender: None,
            city_id: None,
        };

        Ok(Some(r))
    }
}

#[async_trait]
impl UsersAux<Uuid, user::Entity> for Users {
    async fn get_by_login<'a, E: sqlx::Executor<'a, Database = Self::TDatabase>>(
        &self,
        e: E,
        login: &str,
    ) -> DatabaseResult<Uuid> {
        let raw_query = "
            SELECT user_id FROM users WHERE login=$1;
        ";

        let query = sqlx::query(raw_query).bind(login);

        tracing::info!("user repo call exists_by_login with query={}", query.sql());

        let result_query = query
            .fetch_optional(e)
            .await
            .map_err(|e| StoreError::ExecutionFailed(e.to_string()))?;

        let row = result_query
            .ok_or_else(|| StoreError::NoData(String::from("no user with the login")))?;

        tracing::debug!("got the data for auth {row:?}");

        if row.is_empty() {
            return Err(StoreError::NoData(String::from("no the login")));
        }

        let user_id: Uuid = row.get(0);

        return Ok(user_id);
    }
}

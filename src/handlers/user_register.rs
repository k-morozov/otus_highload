use async_trait::async_trait;
use tracing::Level;
use uuid::Uuid;

use crate::dao::base_dao::BaseDao;
use crate::dao::city_aux_dao::CityAuxDao;
use crate::dao::city_entity::CityEntity;
use crate::dao::interest_entity::Builder;
use crate::dao::error::BuilderError;
use crate::dao::interests_aux_dao::InterestsAuxDao;
use crate::dao::pg_city_dao::PgCityDao;
use crate::dao::pg_interests_dao::PgInterestsDao;
use crate::error::ServiceError;
use crate::handlers::handler::Handler;
use crate::model::UserRegisterRequestBody;
use crate::repo::entity::{user, user_credentials, user_interests};
use crate::repo::repo_context::RepoContext;
use crate::repo::repository::Repository;
use crate::store::error::StoreError;
use crate::utils;

pub struct UserRegister;

impl UserRegister {}

#[async_trait]
impl Handler for UserRegister {
    type TModel = UserRegisterRequestBody;
    type TResponse = ();

    async fn process(
        ctx: RepoContext,
        model: Self::TModel,
    ) -> Result<Self::TResponse, ServiceError> {
        tracing::event!(Level::INFO, "UserRegister::process model={:?}", model);

        let mut tx = ctx.get_pool().as_inner_ref().begin().await.map_err(|er| {
            StoreError::TransactionFailed(format!("could not start a transaction {er}"))
        })?;

        let entity = model.clone().into();

        let city_id = {
            let mut city_dao = PgCityDao::new(tx.as_mut());
            let city_res = city_dao.get_id(&entity).await?;

            let city_id = match city_res {
                Some(id) => id,
                None => {
                    let _ = city_dao.create(&entity).await?;
                    let city_res = city_dao.get_id(&entity).await?;

                    city_res.expect("recently was created")
                }
            };
            tracing::info!("got city_id={} for city {}", city_id, entity.city_name);

            city_id
        };

        let entity = model
            .interests
            .iter()
            .fold(Builder::new(), |mut b, interest| {
                let interest_id = Uuid::new_v4();
                b.add_interest(interest_id, interest.clone());

                tracing::info!(
                    "got interest_id={} for interest {:?}",
                    interest_id,
                    interest
                );
                b
            })
            .build()?;

        let mut interests_dao = PgInterestsDao::new(tx.as_mut());
        let _interests_res = interests_dao.create(&entity).await?;

        let user_id = Uuid::new_v4();
        let entity = user::Builder::new()
            .add_model(model.clone())
            .add_user_id(user_id)
            .add_city_id(city_id)
            .build()?;

        let _res = ctx.user_repo().create(tx.as_mut(), &entity).await?;

        let entity = user_credentials::Builder::new()
            .add_user_id(user_id)
            .add_password(&utils::crypto_password(model.password.as_str()))
            .build()?;

        let _ = ctx
            .user_credentials_repo()
            .create(tx.as_mut(), &entity)
            .await?;

        let mut builder = user_interests::Builder::new();
        builder.add_user_id(user_id);

        let mut interests_dao = PgInterestsDao::new(tx.as_mut());
        for interest_name in &model.interests {
            let interest_id = interests_dao.get_id_by_name(interest_name).await?;
            builder.add_interest_id(interest_id);

            tracing::info!(
                "add interest {} with interest_id={} for user_id {}",
                interest_name,
                interest_id,
                user_id
            );
        }
        let entity = builder.build()?;

        let _ = ctx
            .user_interest_repo()
            .create(tx.as_mut(), &entity)
            .await?;

        tx.commit().await.map_err(|er| {
            StoreError::TransactionFailed(format!("could commit a transaction {er}"))
        })?;

        Ok(())
    }
}

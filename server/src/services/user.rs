use bcrypt::BcryptError;
use sea_orm::{
    ActiveValue, ColumnTrait, DbConn, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
};
use uuid::Uuid;

use crate::{
    api_error::ApiError,
    dto::auth::create_user::CreateUserDTO,
    entities::users::{self, Column, Entity as User, Model},
};

impl Model {
    pub async fn create(user: CreateUserDTO, db: &DbConn) -> Result<Option<Model>, ApiError> {
        let data = Model::from(user);

        let new_user = users::ActiveModel {
            id: ActiveValue::set(data.id),
            name: ActiveValue::set(data.name),
            email: ActiveValue::set(data.email),
            password: ActiveValue::set(data.password),
            created_at: ActiveValue::set(data.created_at),
            updated_at: ActiveValue::set(data.updated_at),
            deleted_at: ActiveValue::set(data.deleted_at),
        };

        let new_user = users::Entity::insert(new_user).exec(db).await?;

        let new_user = users::Entity::find_by_id(new_user.last_insert_id)
            .all(db)
            .await?;

        Ok(new_user.first().cloned())
    }

    pub async fn find_by_email(email: &str, db: &DbConn) -> Result<Option<Model>, ApiError> {
        let result: Vec<Model> = User::find()
            .filter(Column::Email.contains(email))
            .order_by_asc(Column::CreatedAt)
            .limit(1)
            .all(db)
            .await?;

        Ok(result.first().cloned())
    }

    pub async fn find(id: &Uuid, db: &DbConn) -> Result<Option<Model>, ApiError> {
        let result: Option<Model> = User::find_by_id(id.to_owned()).one(db).await?;
        Ok(result)
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, BcryptError> {
        Ok(bcrypt::verify(password, &self.password)?)
    }
}

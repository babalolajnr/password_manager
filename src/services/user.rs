use sea_orm::{ActiveValue, DbConn, EntityTrait};

use crate::{
    api_error::ApiError,
    dto::create_user::CreateUserDTO,
    entities::user::{self, Model},
};

impl Model {
    pub async fn create(
        user: CreateUserDTO,
        db: &DbConn,
    ) -> Result<Option<Model>, ApiError> {
        let data = Model::from(user);

        let new_user = user::ActiveModel {
            id: ActiveValue::set(data.id),
            name: ActiveValue::set(data.name),
            email: ActiveValue::set(data.email),
            password: ActiveValue::set(data.password),
            created_at: ActiveValue::set(data.created_at),
            updated_at: ActiveValue::set(data.updated_at),
            deleted_at: ActiveValue::set(data.deleted_at),
        };

        let new_user = user::Entity::insert(new_user).exec(db).await?;

        let new_user = user::Entity::find_by_id(new_user.last_insert_id).all(db).await?;

        Ok(new_user.first().cloned())
    }
}

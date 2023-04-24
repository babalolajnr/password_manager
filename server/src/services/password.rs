use sea_orm::{ColumnTrait, DbConn, EntityTrait, IntoActiveModel, QueryFilter};
use uuid::Uuid;

use crate::{
    api_error::ApiError,
    dto::password::create_password::CreatePasswordDTO,
    entities::passwords::{self, Model as Password},
};

impl Password {
    /// Create a new password
    pub async fn create(
        password: CreatePasswordDTO,
        db: &DbConn,
    ) -> Result<Option<Password>, ApiError> {
        let new_password = Password::from(password).into_active_model();

        let insert_result = passwords::Entity::insert(new_password).exec(db).await?;

        let password = passwords::Entity::find_by_id(insert_result.last_insert_id)
            .one(db)
            .await?
            .map(|p| p.into());

        Ok(password)
    }

    /// Get all passwords for a user
    pub async fn passwords(db: &DbConn, user_id: &Uuid) -> Result<Vec<Password>, ApiError> {
        let passwords = passwords::Entity::find()
            .filter(passwords::Column::UserId.eq(*user_id))
            .all(db)
            .await?;

        Ok(passwords)
    }
}

use sea_orm::{ActiveValue, DbConn, EntityTrait};

use crate::{
    api_error::ApiError,
    dto::password::create_password::CreatePasswordDTO,
    entities::passwords::{self, Model as Password},
};

impl Password {
    pub async fn create(
        password: CreatePasswordDTO,
        db: &DbConn,
    ) -> Result<Option<Password>, ApiError> {
        let Password {
            id,
            user_id,
            website,
            username,
            password,
            note,
            tags,
            url,
            security_question,
            created_at,
            updated_at,
            deleted_at,
            expired_at,
        } = Password::from(password);

        let new_password = passwords::ActiveModel {
            id: ActiveValue::set(id),
            user_id: ActiveValue::set(user_id),
            website: ActiveValue::Set(website),
            username: ActiveValue::Set(username),
            password: ActiveValue::Set(password),
            note: ActiveValue::Set(note),
            tags: ActiveValue::Set(tags),
            url: ActiveValue::Set(url),
            security_question: ActiveValue::Set(security_question),
            created_at: ActiveValue::Set(created_at),
            updated_at: ActiveValue::Set(updated_at),
            deleted_at: ActiveValue::Set(deleted_at),
            expired_at: ActiveValue::Set(expired_at),
        };

        let password = passwords::Entity::insert(new_password).exec(db).await?;

        let password = passwords::Entity::find_by_id(password.last_insert_id)
            .all(db)
            .await?;

        Ok(password.first().cloned())
    }
}

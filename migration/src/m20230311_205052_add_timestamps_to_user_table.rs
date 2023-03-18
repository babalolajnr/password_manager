use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("users"))
                    .add_column_if_not_exists(
                        ColumnDef::new(User::CreatedAt).timestamp().not_null(),
                    )
                    .add_column_if_not_exists(
                        ColumnDef::new(User::UpdatedAt).timestamp().not_null(),
                    )
                    .add_column_if_not_exists(ColumnDef::new(User::DeletedAt).timestamp().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Table::alter()
            .table(Alias::new("users"))
            .drop_column(Alias::new("updated_at"))
            .drop_column(Alias::new("created_at"))
            .drop_column(Alias::new("deleted_at"))
            .to_owned();

        manager.alter_table(stmt).await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

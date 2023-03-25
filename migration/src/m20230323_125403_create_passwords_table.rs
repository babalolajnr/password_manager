use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("passwords"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Password::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Password::UserId).uuid().not_null())
                    .col(ColumnDef::new(Password::Website).string())
                    .col(ColumnDef::new(Password::Username).string().not_null())
                    .col(ColumnDef::new(Password::Password).string().not_null())
                    .col(ColumnDef::new(Password::Note).text())
                    .col(ColumnDef::new(Password::Tags).string())
                    .col(ColumnDef::new(Password::Url).string().not_null())
                    .col(ColumnDef::new(Password::SecurityQuestion).text())
                    .col(ColumnDef::new(Password::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Password::UpdatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Password::DeletedAt).timestamp())
                    .col(ColumnDef::new(Password::ExpiredAt).timestamp())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Alias::new("passwords")).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Password {
    Id,
    UserId,
    Website,
    Username,
    Password,
    Note,
    Tags,
    // SecurityLevel,
    Url,
    SecurityQuestion,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
    ExpiredAt,
}

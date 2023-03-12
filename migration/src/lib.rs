pub use sea_orm_migration::prelude::*;

mod m20230311_082540_create_user_table;
mod m20230311_205052_add_timestamps_to_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230311_082540_create_user_table::Migration),
            Box::new(m20230311_205052_add_timestamps_to_user_table::Migration),
        ]
    }
}

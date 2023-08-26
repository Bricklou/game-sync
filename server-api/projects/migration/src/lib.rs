pub use sea_orm_migration::prelude::*;

pub struct Migrator;

mod m20230820_204552_create_user_table;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230820_204552_create_user_table::Migration),
        ]
    }
}

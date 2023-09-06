pub use sea_orm_migration::prelude::*;

pub struct Migrator;

mod m20230820_204552_create_user_table;
mod m20230904_210741_create_game_table;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230820_204552_create_user_table::Migration),
            Box::new(m20230904_210741_create_game_table::Migration),
        ]
    }
}

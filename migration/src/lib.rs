pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_todolist_table;
mod m20220101_000002_create_todo_table;
mod m20231214_095636_create_user_table;
mod m20231217_062041_add_todolist_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_todolist_table::Migration),
            Box::new(m20220101_000002_create_todo_table::Migration),
            Box::new(m20231214_095636_create_user_table::Migration),
            Box::new(m20231217_062041_add_todolist_user::Migration),
        ]
    }
}

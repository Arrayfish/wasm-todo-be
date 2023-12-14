use sea_orm_migration::prelude::*;

use crate::m20220101_000001_create_todolist_table::TodoList;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Todo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Todo::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Todo::Title).string().not_null())
                    .col(ColumnDef::new(Todo::Completed).string().not_null())
                    .col(ColumnDef::new(Todo::ListId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-todo-todolist_id")
                            .from(Todo::Table, Todo::ListId)
                            .to(TodoList::Table, TodoList::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Todo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Todo {
    Table,
    Id,
    ListId,
    Title,
    Completed,
}

use sea_orm_migration::prelude::*;
// use crate::m20220101_000001_create_todolist_table::TodoList;
use crate::m20231214_095636_create_user_table::User;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(TodoList::Table)
                    .add_column(ColumnDef::new(TodoList::UserId).integer().not_null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk-todolist-user_id")
                            .from_tbl(TodoList::Table)
                            .from_col(TodoList::UserId)
                            .to_tbl(User::Table)
                            .to_col(User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(TodoList::Table)
                    .drop_column(TodoList::UserId)
                    .to_owned(),
            )
            .await
    }
}
#[derive(DeriveIden)]
pub enum TodoList {
    Table,
    Id,
    ListName,
    UserId,
}

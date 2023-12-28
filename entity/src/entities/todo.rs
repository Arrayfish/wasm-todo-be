//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.6

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "todo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub title: String,
    pub completed: String,
    pub list_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::todo_list::Entity",
        from = "Column::ListId",
        to = "super::todo_list::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    TodoList,
}

impl Related<super::todo_list::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TodoList.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

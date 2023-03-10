//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "car")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub brand: String,
    pub model: String,
    pub year: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

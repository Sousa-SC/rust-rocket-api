use std::cmp::{Eq, PartialEq};
use serde::{Deserialize, Serialize};
use generic_crud_proc_macro::CRUDModel;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, CRUDModel)]
#[module = "animal"]
pub struct Animal {
    #[idField] pub id: i32,
    pub race: String,
    pub name: String,
    pub age: i32,
}
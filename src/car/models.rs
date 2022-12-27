use std::cmp::{Eq, PartialEq};
use serde::{Deserialize, Serialize};
use generic_crud_proc_macro::CRUDModel;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, CRUDModel)]
#[module = "car"]
pub struct Car {
    #[idField] pub id: i32,
    pub brand: String,
    pub model: String,
    pub year: i32,
}

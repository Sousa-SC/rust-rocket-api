use async_trait::async_trait;
use sea_orm_rocket::Connection;
use rocket::serde::json::Json;
use rocket::http::uri::Origin;
use rocket::response::status::{Created, NoContent};

use sea_orm::{DatabaseConnection};

use db::Db;

#[async_trait]
pub trait CRUDControllerTrait<Model, CreateModel, PartialModel> {
    async fn reads(conn: Connection<'_, Db>) -> Json<Vec<Model>>;
    async fn read(obj_id: i32, conn: Connection<'_, Db>) -> Option<Json<Model>>;
    async fn post(car: Json<CreateModel>, conn: Connection<'_, Db>, uri: &Origin<'_>) -> Created<Json<Model>>;
    async fn patch(obj_id: i32, car: Json<PartialModel>, conn: Connection<'_, Db>) -> Option<Json<Model>>;
    async fn delete(obj_id: i32, conn: Connection<'_, Db>) -> Option<NoContent>;
}

#[async_trait]
pub trait CRUDServiceTrait<Model, CreateModel, PartialModel> {
    async fn get_all(db: &DatabaseConnection) -> Vec<Model>;
    async fn get_by_id(obj_id: i32, db: &DatabaseConnection) -> Option<Model>;
    async fn create(form: CreateModel, db: &DatabaseConnection) -> Model;
    async fn update(obj_id: i32, form: PartialModel, db: &DatabaseConnection) -> Option<Model>;
    async fn delete(obj_id: i32, db: &DatabaseConnection) -> Option<()>;
}

pub trait FromEntity<EntityModel> {
    fn from_entity(entity: EntityModel) -> Self;
}

pub trait ToActiveModel<ActiveModel, Model> {
    fn into_active_model(self, placeholder: Model) -> ActiveModel;
}
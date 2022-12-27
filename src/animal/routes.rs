use rocket::{Build, Rocket};
use rocket::http::uri::Origin;
use rocket::response::status::{Created, NoContent};
use rocket::serde::json::Json;
use sea_orm_rocket::Connection;

use db::Db;
use generic_crud_trait::CRUDControllerTrait;

use crate::animal;

#[get("/")]
async fn reads(conn: Connection<'_, Db>) -> Json<Vec<animal::models::Animal>> {
    animal::controller::AnimalController::reads(conn).await
}

#[get("/<obj_id>")]
async fn read(obj_id: i32, conn: Connection<'_, Db>) -> Option<Json<animal::models::Animal>> {
    animal::controller::AnimalController::read(obj_id, conn).await
}

#[post("/", data = "<animal>")]
async fn post(animal: Json<animal::models::PostAnimal>, conn: Connection<'_, Db>, uri: &Origin<'_>) -> Created<Json<animal::models::Animal>> {
    animal::controller::AnimalController::post(animal, conn, uri).await
}

#[patch("/<obj_id>", data = "<animal>")]
async fn patch(obj_id: i32, animal: Json<animal::models::PartialAnimal>, conn: Connection<'_, Db>) -> Option<Json<animal::models::Animal>> {
    animal::controller::AnimalController::patch(obj_id, animal, conn).await
}

#[delete("/<obj_id>")]
async fn delete(obj_id: i32, conn: Connection<'_, Db>) -> Option<NoContent> {
    animal::controller::AnimalController::delete(obj_id, conn).await
}


pub fn fuel(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/api/animals", routes![reads, read, post, patch, delete])
}
use rocket::{Build, Rocket};
use rocket::http::uri::Origin;
use rocket::response::status::{Created, NoContent};
use rocket::serde::json::Json;
use sea_orm_rocket::Connection;

use db::Db;
use generic_crud_trait::CRUDControllerTrait;

use crate::car;

#[get("/")]
async fn reads(conn: Connection<'_, Db>) -> Json<Vec<car::models::Car>> {
    car::controller::CarController::reads(conn).await
}

#[get("/<obj_id>")]
async fn read(obj_id: i32, conn: Connection<'_, Db>) -> Option<Json<car::models::Car>> {
    car::controller::CarController::read(obj_id, conn).await
}

#[post("/", data = "<car>")]
async fn post(car: Json<car::models::PostCar>, conn: Connection<'_, Db>, uri: &Origin<'_>) -> Created<Json<car::models::Car>> {
    car::controller::CarController::post(car, conn, uri).await
}

#[patch("/<obj_id>", data = "<car>")]
async fn patch(obj_id: i32, car: Json<car::models::PartialCar>, conn: Connection<'_, Db>) -> Option<Json<car::models::Car>> {
    car::controller::CarController::patch(obj_id, car, conn).await
}

#[delete("/<obj_id>")]
async fn delete(obj_id: i32, conn: Connection<'_, Db>) -> Option<NoContent> {
    car::controller::CarController::delete(obj_id, conn).await
}


pub fn fuel(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/api/cars", routes![reads, read, post, patch, delete])
}
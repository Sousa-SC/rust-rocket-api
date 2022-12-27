#[macro_use] extern crate rocket;

pub mod entity;
pub mod animal;
pub mod car;

use migration::MigratorTrait;
use rocket::fairing::AdHoc;
use sea_orm_rocket::Database;
use db::Db;


#[get("/")]
fn health_check() -> &'static str {
    "OK"
}

async fn run_migrations(rocket: rocket::Rocket<rocket::Build>) -> rocket::fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

fn rocket() -> rocket::Rocket<rocket::Build> {
    let mut rocket = rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/api", routes![health_check]);

    rocket = car::routes::fuel(rocket);
    rocket = animal::routes::fuel(rocket);

    rocket
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenvy::dotenv().ok();

    let _rocket = rocket()
        .ignite().await?
        .launch().await?;

    Ok(())
}
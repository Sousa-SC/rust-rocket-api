# Rocket & Sea-ORM API

This is a simple API built with [Rocket](https://rocket.rs/) and [Sea-ORM](https://www.sea-ql.org/SeaORM/).

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

- Rust version 1.66.0 or higher

### Installing

#### 1. Clone the repository
```shell
git clone  
```

#### 2. Build the project
```shell
cargo build
```

#### 3. Run the project
```shell
cargo run
```

## Usage

Here you will find the steps needed to add a new "Animal" CRUD to the API.

### Create a new sea-orm migration

#### 1. Navigate to the migrations directory
```shell
cd src/migrations
```

#### 2. Create a new migration
```shell
cargo run -- generate create_animal_table
```
This will create a new file with the name you specified.
And modify the lib.rs file in the migrations directory to include the new migration. 

#### 3. Edit the new migration file

The new migration file will contain an up and down implementation for the migration.
You can edit the up and down implementation to include the changes you want to make to the database such as creating a new table or adding a new column to an existing table.

Documentation for the sea-orm migration can be found [here](https://www.sea-ql.org/SeaORM/docs/migration/writing-migration/#defining-migration).

You can also find an example of a migration file by looking the [create_animal_table file](./src/migrations/src/m20221227_085209_create_animal_table.rs).

#### 4. Run the migration
```shell
cargo run -- up
```

Additional commands example can be found in the [migrations README](./src/migrations/README.md).

### Generate the new entity files

#### 1. Navigate to the entity directory
```shell
cd src/entity
```

#### 2. Generate the new entity files
```shell
sea-orm-cli generate entity --with-serde both
```
This will create a new file with the name you specified. In this example: animal.rs

Note: You need to have the sea-orm-cli installed. You can install it with the following command:
```shell
cargo install sea-orm-cli
```

### Create the new module for to handle animal requests

#### 1. Navigate to the root directory
```shell
cd project_directory
```

#### 2. Create a new animal module

Here is an example tree of the animal module
```
src
└── animal
    ├── mod.rs
    ├── controller.rs
    ├── models.rs
    ├── routes.rs
    └── service.rs
```
Note: These files are mandatory for the animal module to work and the names are important.
This will be used by the provided rust macros to generate the code needed to handle a simple CRUD.

Touch the files with the following command:
```shell
mkdir src/animal
touch src/animal/{mod.rs,controller.rs,models.rs,routes.rs,service.rs}
```

<br>

###### mod.rs
Here we simply publish the files in the animal directory.
```rust
pub mod controller;
pub mod routes;
pub mod service;
pub mod models;
```
<br>

###### controller.rs
Here we define the controller for the animal module.

It uses the CRUDControllerImpl macro to generate the code needed to handle a simple CRUD.
```rust
use generic_crud_proc_macro::CRUDControllerImpl;

#[derive(CRUDControllerImpl)]
#[module = "animal"]
pub struct AnimalController;
```
You can find the trait generated by the macro [here](./src/generic_crud/trait/trait.rs).

<br>

###### models.rs
Here we define the models for the animal module with the same fields as the animal table in the database.

It uses the CRUDModel macro to generate the code needed to handle a simple CRUD.

We also need to add the #[idField] attribute to the id field so that the CRUDModel macro knows which field is the id field.
```rust
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
```

<br>

###### service.rs
Here we define the service for the animal module.
```rust
use generic_crud_proc_macro::CRUDServiceImpl;

#[derive(CRUDServiceImpl)]
#[module = "animal"]
pub struct AnimalService;
```

You can find the trait generated by the macro [here](./src/generic_crud/trait/trait.rs).

<br>

###### routes.rs
Finally, we define the routes for the animal module.

In this example we will define the 5 routes provided by the CRUDControllerImpl macro.

```rust
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
```

<br>

###### main.rs
Finally, we need to fuel our rocket with the animal routes.

We need to use the animal module in the main.rs file.
```rust
pub mod animal;
```

And we need to add the animal routes to the rocket in the rocket() fn.
```rust
rocket = animal::routes::fuel(rocket);
```

git init
git add -all
git commit -m "car and animal CRUD, generic CRUD, and readme explanation"
git branch -M master
git remote add origin git@github.com:Sousa-SC/rust-rocket-api.git
git push -u origin master
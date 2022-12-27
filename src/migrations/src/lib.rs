pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_cars_table;
mod m20221227_085209_create_animal_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_cars_table::Migration),
            Box::new(m20221227_085209_create_animal_table::Migration),
        ]
    }
}

pub use sea_orm_migration::prelude::*;

mod m20250122_155430_create_uom_table;
mod m20250128_102113_create_category_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
  fn migrations() -> Vec<Box<dyn MigrationTrait>> {
    vec![
            Box::new(m20250122_155430_create_uom_table::Migration),
            Box::new(m20250128_102113_create_category_table::Migration),
        ]
  }
}

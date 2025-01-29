use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(AttributeOptionValue::Table)
          .if_not_exists()
          .col(uuid(AttributeOptionValue::Id).primary_key())
          .col(text(AttributeOptionValue::Name).default(""))
          .col(uuid(AttributeOptionValue::AttributeOptionId))
          .foreign_key(
            ForeignKey::create()
              .name("fk-attribute_option_value-attribute_option_id")
              .from(
                AttributeOptionValue::Table,
                AttributeOptionValue::AttributeOptionId,
              )
              .to(AttributeOption::Table, AttributeOption::Id),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(AttributeOptionValue::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum AttributeOptionValue {
  Table,
  Id,
  Name,
  AttributeOptionId,
}

#[derive(DeriveIden)]
enum AttributeOption {
  Table,
  Id,
}

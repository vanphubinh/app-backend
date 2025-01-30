use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .create_table(
        Table::create()
          .table(VariantCombination::Table)
          .if_not_exists()
          .col(uuid(VariantCombination::ProductId))
          .col(uuid(VariantCombination::AttributeOptionId))
          .primary_key(
            Index::create()
              .name("pk-variant_combination")
              .col(VariantCombination::ProductId)
              .col(VariantCombination::AttributeOptionId),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-variant_combination-product_id")
              .from(VariantCombination::Table, VariantCombination::ProductId)
              .to(Product::Table, Product::Id),
          )
          .foreign_key(
            ForeignKey::create()
              .name("fk-variant_combination-attribute_option_id")
              .from(
                VariantCombination::Table,
                VariantCombination::AttributeOptionId,
              )
              .to(AttributeOption::Table, AttributeOption::Id),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(VariantCombination::Table).to_owned())
      .await
  }
}

#[derive(DeriveIden)]
enum VariantCombination {
  Table,
  ProductId,
  AttributeOptionId,
}

#[derive(DeriveIden)]
enum Product {
  Table,
  Id,
}

#[derive(DeriveIden)]
enum AttributeOption {
  Table,
  Id,
}

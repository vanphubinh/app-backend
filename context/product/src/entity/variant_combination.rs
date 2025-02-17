use async_trait::async_trait;
use infra::uuid::Uuid;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "variant_combination")]
#[serde(rename_all = "camelCase")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub product_id: Uuid,
  #[sea_orm(primary_key, auto_increment = false)]
  pub attribute_option_value_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::product::Entity",
    from = "Column::ProductId",
    to = "super::product::Column::Id"
  )]
  Product,
  #[sea_orm(
    belongs_to = "super::attribute_option_value::Entity",
    from = "Column::AttributeOptionValueId",
    to = "super::attribute_option_value::Column::Id"
  )]
  AttributeOptionValue,
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {}

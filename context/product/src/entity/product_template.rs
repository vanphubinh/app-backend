use async_trait::async_trait;
use chrono::Utc;
use infra::uuid::Uuid;
use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "product_template")]
#[serde(rename_all = "camelCase")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  #[sea_orm(column_type = "Text")]
  pub name: String,
  #[sea_orm(column_type = "Text")]
  pub description: String,
  pub uom_id: Uuid,
  #[sea_orm(nullable)]
  pub category_id: Option<Uuid>,
  pub product_type: ProductType,
  pub product_subtype: ProductSubtype,
  pub is_track_inventory: bool,
  pub created_at: ChronoDateTimeWithTimeZone,
  #[sea_orm(nullable)]
  pub updated_at: Option<ChronoDateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::product::Entity")]
  Product,
}

impl Related<super::product::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Product.def()
  }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
  fn new() -> Self {
    Self {
      id: Set(Uuid::new()),
      ..ActiveModelTrait::default()
    }
  }

  async fn before_save<C>(self, db: &C, insert: bool) -> Result<Self, DbErr>
  where
    C: ConnectionTrait,
  {
    let _ = db;
    let mut this = self;
    if !insert {
      this.updated_at = Set(Some(Utc::now().into()));
    }
    Ok(this)
  }
}

#[derive(
  Debug, EnumIter, DeriveActiveEnum, Deserialize, Clone, PartialEq, Eq, Serialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "product_type")]
#[serde(rename_all = "lowercase")]
pub enum ProductType {
  #[sea_orm(string_value = "goods")]
  #[serde(rename = "goods")]
  Goods,
  #[sea_orm(string_value = "service")]
  #[serde(rename = "service")]
  Service,
}

#[derive(
  Debug, EnumIter, DeriveActiveEnum, Deserialize, Clone, PartialEq, Eq, Serialize, ToSchema,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "product_subtype")]
#[serde(rename_all = "lowercase")]
pub enum ProductSubtype {
  #[sea_orm(string_value = "normal")]
  #[serde(rename = "normal")]
  Normal,
  #[sea_orm(string_value = "packaging_with_print")]
  #[serde(rename = "packaging_with_print")]
  PackagingWithPrint,
  #[sea_orm(string_value = "mould")]
  #[serde(rename = "mould")]
  Mould,
}

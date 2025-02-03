use crate::{
  dto::category::Category,
  entity::product_template::{ProductSubtype, ProductType},
};
use infra::uuid::Uuid;
use measurement::dto::uom::Uom;
use sea_orm::{prelude::Decimal, FromQueryResult};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, FromQueryResult)]
pub struct ProductQueryResult {
  pub id: Uuid,
  pub name: String,
  pub product_template_id: Uuid,
  pub category_id: Option<Uuid>,
  pub category_name: Option<String>,
  pub uom_id: Uuid,
  pub uom_name: String,
  pub is_track_inventory: bool,
  pub product_type: ProductType,
  pub product_subtype: ProductSubtype,
  pub price: Decimal,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Product {
  pub id: Uuid,
  pub name: String,
  #[schema(rename = "productTemplateId")]
  pub product_template_id: Uuid,
  pub category: Option<Category>,
  pub uom: Uom,
  #[schema(rename = "isTrackInventory")]
  pub is_track_inventory: bool,
  #[schema(rename = "productType")]
  pub product_type: ProductType,
  #[schema(rename = "productSubtype")]
  pub product_subtype: ProductSubtype,
  #[schema(rename = "price")]
  pub price: Decimal,
}

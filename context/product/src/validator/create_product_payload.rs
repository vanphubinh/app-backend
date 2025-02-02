use infra::uuid::Uuid;
use sea_orm::prelude::Decimal;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::entity::product_template::{ProductSubtype, ProductType};

#[derive(Deserialize, ToSchema)]
pub struct CreateProductPayload {
  #[schema(example = "Sản phẩm 1")]
  pub name: String,

  #[schema(example = true, rename = "isSellable")]
  #[serde(rename(deserialize = "isSellable"))]
  pub is_sellable: bool,

  #[schema(example = false, rename = "isPurchasable")]
  #[serde(rename(deserialize = "isPurchasable"))]
  pub is_purchasable: bool,

  #[schema(example = true, rename = "isTrackInventory")]
  #[serde(rename(deserialize = "isTrackInventory"))]
  pub is_track_inventory: bool,

  #[schema(example = "1cjBf8J9HvUQxtPimwpDLF", rename = "uomId")]
  #[serde(rename(deserialize = "uomId"))]
  pub uom_id: Uuid,

  #[schema(example = "1cjBf8J9HvUQxtPimwpDLF", rename = "categoryId")]
  #[serde(rename(deserialize = "categoryId"))]
  pub category_id: Option<Uuid>,

  #[schema(example = "goods", rename = "productType")]
  #[serde(rename(deserialize = "productType"))]
  pub product_type: ProductType,

  #[schema(example = "normal", rename = "productSubtype")]
  #[serde(rename(deserialize = "productSubtype"))]
  pub product_subtype: ProductSubtype,

  #[schema(example = 50, rename = "price")]
  pub price: Decimal,

  #[schema(example = "This is a product")]
  pub description: String,
}

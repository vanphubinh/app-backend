use crate::entity::uom;
use infra::uuid::Uuid;
use sea_orm::{DerivePartialModel, FromQueryResult, ModelTrait};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, DerivePartialModel, Serialize, FromQueryResult, ToSchema)]
#[sea_orm(entity = "<uom::Model as ModelTrait>::Entity")]
#[serde(rename_all = "camelCase")]
pub struct Uom {
  #[schema(example = "123e4567")]
  pub id: Uuid,
  #[schema(example = "kg")]
  pub name: String,
}

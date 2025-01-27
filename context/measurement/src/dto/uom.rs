use crate::entity::uom;
use infra::uuid::Uuid;
use sea_orm::{DerivePartialModel, FromQueryResult, ModelTrait};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, DerivePartialModel, Serialize, FromQueryResult, ToSchema)]
#[sea_orm(entity = "<uom::Model as ModelTrait>::Entity")]
#[schema(rename_all = "camelCase")]
pub struct Uom {
  #[schema(example = "1cjBf8J9HvUQxtPimwpDLF")]
  pub id: Uuid,
  #[schema(example = "kg")]
  pub name: String,
}

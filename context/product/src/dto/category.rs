use crate::entity::category;
use infra::uuid::Uuid;
use sea_orm::{DerivePartialModel, FromQueryResult, ModelTrait};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, DerivePartialModel, Serialize, FromQueryResult, ToSchema)]
#[sea_orm(entity = "<category::Model as ModelTrait>::Entity")]
#[schema(rename_all = "camelCase")]
pub struct Category {
  #[schema(example = "1cjBf8J9HvUQxtPimwpDLF")]
  pub id: Uuid,
  #[schema(example = "packaging")]
  pub name: String,
}

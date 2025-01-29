use crate::entity::attribute_option;
use infra::uuid::Uuid;
use sea_orm::{DerivePartialModel, FromQueryResult, ModelTrait};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, DerivePartialModel, Serialize, FromQueryResult, ToSchema)]
#[sea_orm(entity = "<attribute_option::Model as ModelTrait>::Entity")]
#[schema(rename_all = "camelCase")]
pub struct AttributeOption {
  #[schema(example = "1cjBf8J9HvUQxtPimwpDLF")]
  pub id: Uuid,
  #[schema(example = "color")]
  pub name: String,
}

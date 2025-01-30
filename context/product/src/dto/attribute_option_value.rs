use crate::entity::attribute_option_value;
use infra::uuid::Uuid;
use sea_orm::{DerivePartialModel, FromQueryResult, ModelTrait};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, DerivePartialModel, Serialize, FromQueryResult, ToSchema)]
#[sea_orm(entity = "<attribute_option_value::Model as ModelTrait>::Entity")]
#[schema(rename_all = "camelCase")]
pub struct AttributeOptionValue {
  #[schema(example = "1cjBf8J9HvUQxtPimwpDLF")]
  pub id: Uuid,
  #[schema(example = "red")]
  pub value: String,
}

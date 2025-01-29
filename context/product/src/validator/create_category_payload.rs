use infra::uuid::Uuid;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateCategoryPayload {
  #[schema(example = "kg")]
  pub name: String,

  #[schema(
    example = "1cjBf8J9HvUQxtPimwpDLF",
    rename = "parentCategoryId",
    nullable
  )]
  pub parent_category_id: Option<Uuid>,
}

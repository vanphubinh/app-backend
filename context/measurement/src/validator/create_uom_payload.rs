use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateUomPayload {
  #[schema(example = "kg")]
  pub name: String,
}

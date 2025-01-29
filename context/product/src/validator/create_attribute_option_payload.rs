use serde::Deserialize;
use utoipa::ToSchema;
use utoipauto::utoipa_ignore;

#[derive(Deserialize, ToSchema)]
pub struct CreateAttributeOptionPayload {
  #[schema(example = "color")]
  pub name: String,

  #[schema(
    example = json!([{"name": "red"}, {"name": "blue"}]),
    inline = true,
    rename = "optionValues"
  )]
  #[serde(rename(deserialize = "optionValues"))]
  pub option_values: Vec<OptionValue>,
}

#[utoipa_ignore]
#[derive(Debug, Deserialize, ToSchema)]
pub struct OptionValue {
  pub name: String,
}

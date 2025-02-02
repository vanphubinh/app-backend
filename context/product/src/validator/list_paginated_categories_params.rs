use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListPaginatedCategoriesParams {
  #[param(example = "1", minimum = 1)]
  pub page: Option<u64>,
  #[param(rename = "perPage", example = "30", minimum = 30)]
  pub per_page: Option<u64>,
  #[param(example = "Packaging")]
  pub search: Option<String>,
}

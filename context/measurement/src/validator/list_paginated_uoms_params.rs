use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListPaginatedUomsParams {
  pub page: Option<u64>,
  pub per_page: Option<u64>,
}

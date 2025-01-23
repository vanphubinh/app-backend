use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationMeta {
  pub page: u64,
  pub per_page: u64,
  pub total: u64,
  pub total_pages: u64,
}

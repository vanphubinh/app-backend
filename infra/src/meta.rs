use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginationMeta {
  pub page: u64,
  pub per_page: u64,
  pub total: u64,
  pub total_pages: u64,
}

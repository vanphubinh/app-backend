use serde::Serialize;
use utoipa::{ToResponse, ToSchema};

use super::{meta::PaginationMeta, uuid::Uuid};

#[derive(Serialize)]
pub struct OkResponse {
  ok: bool,
}

impl Default for OkResponse {
  fn default() -> Self {
    OkResponse { ok: true }
  }
}

#[derive(Serialize, ToResponse)]
pub struct OkResponseWithReturningId {
  #[response(example = "true")]
  pub ok: bool,
  #[response(example = "1cjBf8J9HvUQxtPimwpDLF")]
  pub id: Uuid,
}

impl OkResponseWithReturningId {
  pub fn new(id: Uuid) -> Self {
    OkResponseWithReturningId { ok: true, id }
  }
}

#[derive(Serialize)]
pub struct OkResponseWithData<T> {
  pub ok: bool,
  pub data: T,
}

impl<T> OkResponseWithData<T> {
  pub fn new(data: T) -> Self {
    OkResponseWithData { ok: true, data }
  }
}

#[derive(Serialize, ToResponse)]
pub struct PaginatedResponse<T: ToSchema> {
  #[response(example = "true")]
  pub ok: bool,
  pub data: Vec<T>,
  pub meta: PaginationMeta,
}

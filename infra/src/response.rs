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

#[derive(Serialize, ToResponse, ToSchema)]
pub struct OkResponseWithReturningId {
  #[response(example = "true")]
  #[schema(example = "true")]
  pub ok: bool,
  #[response(example = "1cjBf8J9HvUQxtPimwpDLF")]
  #[schema(example = "1cjBf8J9HvUQxtPimwpDLF")]
  pub id: Uuid,
}

impl OkResponseWithReturningId {
  pub fn new(id: Uuid) -> Self {
    OkResponseWithReturningId { ok: true, id }
  }
}

#[derive(Serialize, ToResponse)]
pub struct OkResponseWithArrayData<T: ToSchema> {
  #[response(example = "true")]
  pub ok: bool,
  #[response(example = "{}")]
  pub data: Vec<T>,
}

impl<T: ToSchema> OkResponseWithArrayData<T> {
  pub fn new(data: Vec<T>) -> Self {
    OkResponseWithArrayData { ok: true, data }
  }
}

#[derive(Serialize, ToResponse)]
pub struct PaginatedResponse<T: ToSchema> {
  #[response(example = "true")]
  pub ok: bool,
  pub data: Vec<T>,
  pub meta: PaginationMeta,
}

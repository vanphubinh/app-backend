use serde::Serialize;

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

#[derive(Serialize)]
pub struct OkResponseWithReturningId {
  pub ok: bool,
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

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
  pub ok: bool,
  pub data: Vec<T>,
  pub meta: PaginationMeta,
}

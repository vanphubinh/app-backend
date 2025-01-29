use crate::{route, RouterTrait};
use product::{
  dto::attribute_option::AttributeOption as AttributeOptionDto, service::ProductService,
  validator::ListPaginatedAttributeOptionsParams,
};
use std::sync::Arc;

use axum::{
  extract::{Query, State},
  routing::get,
  Json, Router,
};
use infra::{app_state::AppState, error::AppError, response::PaginatedResponse};

pub struct AttributeOptionRouter;

impl RouterTrait for AttributeOptionRouter {
  fn generate_routes() -> Router<Arc<AppState>> {
    Router::new().merge(list_paginated_attribute_options())
  }
}

#[utoipa::path(
  get,
  path = "/attribute-options/list",
  description = "List paginated attribute options",
  tag = "Product",
  params(ListPaginatedAttributeOptionsParams),
  responses(
    (status = 200, response = inline(PaginatedResponse<AttributeOptionDto>))
  )
)]
fn list_paginated_attribute_options() -> Router<Arc<AppState>> {
  async fn handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ListPaginatedAttributeOptionsParams>,
  ) -> Result<Json<PaginatedResponse<AttributeOptionDto>>, AppError> {
    let (attribute_options, meta) =
      ProductService::list_attribute_options(&state.db.clone(), params).await?;

    Ok(Json(PaginatedResponse::<AttributeOptionDto> {
      ok: true,
      data: attribute_options,
      meta,
    }))
  }

  route("/attribute-options/list", get(handler))
}

use crate::{route, RouterTrait};
use product::{
  dto::attribute_option::AttributeOption as AttributeOptionDto,
  service::ProductService,
  validator::{CreateAttributeOptionPayload, ListPaginatedAttributeOptionsParams},
};
use std::sync::Arc;

use axum::{
  extract::{Query, State},
  routing::{get, post},
  Json, Router,
};
use infra::{
  app_state::AppState,
  error::AppError,
  response::{OkResponseWithReturningId, PaginatedResponse},
};

pub struct AttributeOptionRouter;

impl RouterTrait for AttributeOptionRouter {
  fn generate_routes() -> Router<Arc<AppState>> {
    Router::new()
      .merge(list_paginated_attribute_options())
      .merge(create_attribute_option())
  }
}

#[utoipa::path(
  get,
  path = "/attributeOptions/list",
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

  route("/attributeOptions/list", get(handler))
}

#[utoipa::path(
  post,
  path = "/attributeOptions/create",
  description = "Create attribute option",
  tag = "Product",
  request_body = CreateAttributeOptionPayload,
  responses(
    (status = 200, response = OkResponseWithReturningId)
  )
)]
fn create_attribute_option() -> Router<Arc<AppState>> {
  async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateAttributeOptionPayload>,
  ) -> Result<Json<OkResponseWithReturningId>, AppError> {
    let attribute_option =
      ProductService::create_attribute_option(&state.db.clone(), payload).await?;

    Ok(Json(OkResponseWithReturningId {
      ok: true,
      id: attribute_option.id,
    }))
  }

  route("/attributeOptions/create", post(handler))
}

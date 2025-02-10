use crate::{route, RouterTrait};
use product::{
  dto::attribute_option::AttributeOption as AttributeOptionDto,
  dto::attribute_option_value::AttributeOptionValue as AttributeOptionValueDto,
  service::ProductService,
  validator::{CreateAttributeOptionPayload, ListPaginatedAttributeOptionsParams},
};
use std::sync::Arc;

use axum::{
  extract::{Path, Query, State},
  routing::{get, post},
  Json, Router,
};
use infra::{
  app_state::AppState,
  error::AppError,
  response::{OkResponseWithArrayData, OkResponseWithReturningId, PaginatedResponse},
  uuid::Uuid,
};

pub struct AttributeOptionRouter;

impl RouterTrait for AttributeOptionRouter {
  fn generate_routes() -> Router<Arc<AppState>> {
    Router::new()
      .merge(list_paginated_attribute_options())
      .merge(create_attribute_option())
      .merge(list_option_values_by_attribute_option_id())
  }
}

#[utoipa::path(
  get,
  path = "/attribute_options/list",
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

  route("/attribute_options/list", get(handler))
}

#[utoipa::path(
  post,
  path = "/attribute_options/create",
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

  route("/attribute_options/create", post(handler))
}

#[utoipa::path(
  get,
  path = "/attribute_options/{id}/list_option_values",
  description = "List attribute option values",
  tag = "Product",
  params(
    ("id" = Uuid, Path, description = "Attribute option id", example = "1cjNXfNTJq37QJiL9WTFNQ")
  ),
  responses(
    (status = 200, response = inline(OkResponseWithArrayData<AttributeOptionValueDto>))
  )
)]
fn list_option_values_by_attribute_option_id() -> Router<Arc<AppState>> {
  async fn handler(
    State(state): State<Arc<AppState>>,
    Path(attribute_option_id): Path<Uuid>,
  ) -> Result<Json<OkResponseWithArrayData<AttributeOptionValueDto>>, AppError> {
    let option_values = ProductService::find_option_values_by_attribute_option_id(
      &state.db.clone(),
      attribute_option_id,
    )
    .await?;
    Ok(Json(OkResponseWithArrayData::new(option_values)))
  }

  route("/attribute_options/{id}/list_option_values", get(handler))
}

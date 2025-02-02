use crate::{route, RouterTrait};
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
use measurement::{
  dto::uom::Uom as UomDto,
  service::MeasurementService,
  validator::{CreateUomPayload, ListPaginatedUomsParams},
};

pub struct UomRouter;

impl RouterTrait for UomRouter {
  fn generate_routes() -> Router<Arc<AppState>> {
    Router::new()
      .merge(list_paginated_uoms())
      .merge(create_uom())
  }
}

#[utoipa::path(
  get,
  path = "/uoms/list",
  tag = "Measurement",
  params(ListPaginatedUomsParams),
  responses(
    (status = 200, response = inline(PaginatedResponse<UomDto>))
  )
)]
fn list_paginated_uoms() -> Router<Arc<AppState>> {
  async fn handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ListPaginatedUomsParams>,
  ) -> Result<Json<PaginatedResponse<UomDto>>, AppError> {
    let (uoms, meta) = MeasurementService::list_paginated_uoms(&state.db.clone(), params).await?;

    Ok(Json(PaginatedResponse::<UomDto> {
      ok: true,
      data: uoms,
      meta,
    }))
  }

  route("/uoms/list", get(handler))
}

#[utoipa::path(
  post,
  path = "/uoms/create",
  tag = "Measurement",
  request_body = CreateUomPayload,
  responses(
    (status = 200, response = OkResponseWithReturningId)
  )
)]
fn create_uom() -> Router<Arc<AppState>> {
  async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUomPayload>,
  ) -> Result<Json<OkResponseWithReturningId>, AppError> {
    let uom = MeasurementService::create_uom(&state.db.clone(), payload).await?;
    Ok(Json(OkResponseWithReturningId::new(uom.id)))
  }

  route("/uoms/create", post(handler))
}

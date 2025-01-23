use crate::{route, RouterTrait};
use std::sync::Arc;

use axum::{
  extract::{Query, State},
  routing::get,
  Json, Router,
};
use infra::{app_state::AppState, error::AppError, response::PaginatedResponse};
use measurement::{dto::uom::Uom, service::MeasurementService, validator::ListPaginatedUomsParams};

pub struct UomRouter;

impl RouterTrait for UomRouter {
  fn generate_routes() -> Router<Arc<AppState>> {
    Router::new().merge(list_uoms())
  }
}

fn list_uoms() -> Router<Arc<AppState>> {
  async fn handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ListPaginatedUomsParams>,
  ) -> Result<Json<PaginatedResponse<Uom>>, AppError> {
    let (uoms, meta) = MeasurementService::list_paginated_uoms(&state.db.clone(), params).await?;

    Ok(Json(PaginatedResponse {
      ok: true,
      data: uoms,
      meta,
    }))
  }

  route("/uoms.list", get(handler))
}

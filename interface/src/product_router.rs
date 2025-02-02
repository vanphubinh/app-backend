use crate::{route, RouterTrait};
use std::sync::Arc;

use axum::{extract::State, routing::post, Json, Router};
use infra::{app_state::AppState, error::AppError, response::OkResponseWithReturningId};
use product::{service::ProductService, validator::CreateProductPayload};

pub struct ProductRouter;

impl RouterTrait for ProductRouter {
  fn generate_routes() -> Router<Arc<AppState>> {
    Router::new().merge(create_product())
  }
}

#[utoipa::path(
  post,
  path = "/products/create",
  tag = "Product",
  request_body = CreateProductPayload,
  responses(
    (status = 200, response = OkResponseWithReturningId)
  )
)]
fn create_product() -> Router<Arc<AppState>> {
  async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateProductPayload>,
  ) -> Result<Json<OkResponseWithReturningId>, AppError> {
    let product = ProductService::create_product(&state.db.clone(), payload).await?;
    Ok(Json(OkResponseWithReturningId::new(product.id)))
  }

  route("/products/create", post(handler))
}

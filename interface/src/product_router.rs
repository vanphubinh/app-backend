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
use product::{
  dto::product::Product as ProductDto,
  service::ProductService,
  validator::{CreateProductPayload, ListPaginatedProductsParams},
};

pub struct ProductRouter;

impl RouterTrait for ProductRouter {
  fn generate_routes() -> Router<Arc<AppState>> {
    Router::new()
      .merge(list_paginated_products())
      .merge(create_product())
  }
}

#[utoipa::path(
  get,
  path = "/products/list",
  description = "List paginated products",
  tag = "Product",
  params(ListPaginatedProductsParams),
  responses(
    (status = 200, response = inline(PaginatedResponse<ProductDto>))
  )
)]
fn list_paginated_products() -> Router<Arc<AppState>> {
  async fn handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ListPaginatedProductsParams>,
  ) -> Result<Json<PaginatedResponse<ProductDto>>, AppError> {
    let (products, meta) =
      ProductService::list_paginated_products(&state.db.clone(), params).await?;

    Ok(Json(PaginatedResponse::<ProductDto> {
      ok: true,
      data: products,
      meta,
    }))
  }

  route("/products/list", get(handler))
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

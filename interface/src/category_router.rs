use crate::{route, RouterTrait};
use product::{
  dto::category::Category as CategoryDto,
  service::ProductService,
  validator::{CreateCategoryPayload, ListPaginatedCategoriesParams},
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

pub struct CategoryRouter;

impl RouterTrait for CategoryRouter {
  fn generate_routes() -> Router<Arc<AppState>> {
    Router::new()
      .merge(list_paginated_categories())
      .merge(create_category())
  }
}

#[utoipa::path(
  get,
  path = "/categories/list",
  description = "List paginated categories",
  tag = "Product",
  params(ListPaginatedCategoriesParams),
  responses(
    (status = 200, response = inline(PaginatedResponse<CategoryDto>))
  )
)]
fn list_paginated_categories() -> Router<Arc<AppState>> {
  async fn handler(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ListPaginatedCategoriesParams>,
  ) -> Result<Json<PaginatedResponse<CategoryDto>>, AppError> {
    let (categories, meta) =
      ProductService::list_paginated_categories(&state.db.clone(), params).await?;

    Ok(Json(PaginatedResponse::<CategoryDto> {
      ok: true,
      data: categories,
      meta,
    }))
  }

  route("/categories/list", get(handler))
}

#[utoipa::path(
  post,
  path = "/categories/create",
  description = "Create a category",
  tag = "Product",
  request_body = CreateCategoryPayload,
  responses(
    (status = 200, response = OkResponseWithReturningId)
  )
)]
fn create_category() -> Router<Arc<AppState>> {
  async fn handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCategoryPayload>,
  ) -> Result<Json<OkResponseWithReturningId>, AppError> {
    let category = ProductService::create_category(&state.db.clone(), payload).await?;

    Ok(Json(OkResponseWithReturningId::new(category.id)))
  }

  route("/categories/create", post(handler))
}

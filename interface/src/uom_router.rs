use crate::RouterTrait;

use axum::Router;
use infra::app_state::AppState;
use std::sync::Arc;

pub struct UomRouter;

impl RouterTrait for UomRouter {
  fn generate_routes() -> Router<Arc<AppState>> {
    Router::new()
  }
}

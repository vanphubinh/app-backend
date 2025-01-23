mod uom_router;

use axum::{routing::MethodRouter, Router};
use infra::app_state::AppState;
use std::sync::Arc;

pub use uom_router::UomRouter;

pub trait RouterTrait {
  fn generate_routes() -> Router<Arc<AppState>>;
}

fn route(path: &str, method_router: MethodRouter<Arc<AppState>>) -> Router<Arc<AppState>> {
  Router::new().route(path, method_router)
}

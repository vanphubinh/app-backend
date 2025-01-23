mod uom_router;

use axum::Router;
use infra::app_state::AppState;
use std::sync::Arc;

pub use uom_router::UomRouter;

pub trait RouterTrait {
  fn generate_routes() -> Router<Arc<AppState>>;
}

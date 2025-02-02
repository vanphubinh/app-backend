use axum::{routing::get, Json, Router};
use infra::{app_state::AppState, database};
use interface::{AttributeOptionRouter, CategoryRouter, ProductRouter, RouterTrait, UomRouter};
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};
use utoipauto::utoipauto;

#[tokio::main]
pub async fn start() {
  dotenvy::dotenv().ok();

  #[utoipauto(paths = "./interface/src from interface,
      ./infra/src from infra,
      ./context/measurement/src from measurement,
      ./context/product/src from product")]
  #[derive(OpenApi)]
  #[openapi()]
  struct ApiDoc;

  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        format!(
          "{}=debug,tower_http=debug,axum::rejection=trace",
          env!("CARGO_CRATE_NAME")
        )
        .into()
      }),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

  tracing::info!("Connecting to database...");
  let db = match database::connect_db("DATABASE_URL").await {
    Ok(db) => {
      tracing::info!("Connected to database!");
      db
    }
    Err(_) => {
      tracing::error!("Failed to connect to database!");
      return;
    }
  };

  async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
  }

  let uom_router = UomRouter::generate_routes();
  let category_router = CategoryRouter::generate_routes();
  let attribute_option_router = AttributeOptionRouter::generate_routes();
  let product_router = ProductRouter::generate_routes();

  let app_state = Arc::new(AppState::new(db));

  let app = Router::new()
    .merge(uom_router)
    .merge(category_router)
    .merge(attribute_option_router)
    .merge(product_router)
    .route("/", get(|| async { "Hello, world!" }))
    .route("/docs.json", get(openapi))
    .merge(Scalar::with_url("/docs", ApiDoc::openapi()))
    .layer(TraceLayer::new_for_http())
    .with_state(app_state);

  let port: u16 = std::env::var("PORT")
    .unwrap_or_else(|_| "3000".to_string())
    .parse()
    .expect("Failed to parse PORT");

  let address = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port));
  let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

  tracing::info!("Server is running on {}", address);
  axum::serve(listener, app).await.unwrap();
}

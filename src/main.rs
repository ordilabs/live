use cfg_if::cfg_if;

pub mod types;

cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::app::*;
    use leptos::*;
    mod backend;
    extern crate ord_labs;
    use ord_labs::Inscription;
    use std::collections::HashMap;
    extern crate dotenv;
    extern crate leptos_axum;
    extern crate num_format;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    mod fallback;
    use crate::backend::BitcoinCore;

    use axum::body::Body as AxumBody;
    use axum::{
        extract::{Extension, FromRef, Path, State},
        http::Request,
        response::{IntoResponse, Response},
        routing::{get, post},
        Router,
    };

    pub mod error_template;
    pub mod errors;

    mod server_actions;

    use axum::{
        extract::TypedHeader,
        response::sse::{Event, Sse},
    };
    use futures::stream;
    use std::{convert::Infallible, time::Duration};
    use tokio_stream::StreamExt as _;
    use tower_http::trace::TraceLayer;
    use tracing_subscriber::prelude::*;




    use std::sync::Arc;

    #[derive(Clone)]
    pub(crate) struct HttpClient {}

    #[derive(Clone)]
    pub(crate) struct Database {}

    #[derive(Clone)]
    pub(crate) struct AppState {
        pub(crate) core: backend::bitcoin_core::BitcoinCore,
        pub(crate) _client: HttpClient,
        pub(crate) _db: Database,
    }

    // support converting an `AppState` in an `ApiState`
    impl FromRef<AppState> for HttpClient {
        fn from_ref(app_state: &AppState) -> HttpClient {
            app_state._client.clone()
        }
    }

    // support converting an `AppState` in an `ApiState`
    impl FromRef<AppState> for BitcoinCore {
        fn from_ref(app_state: &AppState) -> BitcoinCore {
            app_state.core.clone()
        }
    }
}}
mod app;

//Define a handler to test extractor with state
#[cfg(feature = "ssr")]
async fn custom_handler(
  Path(id): Path<String>,
  Extension(options): Extension<Arc<LeptosOptions>>,
  req: Request<AxumBody>,
) -> Response {
  let handler = leptos_axum::render_app_to_stream_with_context(
    (*options).clone(),
    move |cx| {
      provide_context(cx, id.clone());
    },
    |cx| view! { cx, <App/> },
  );
  handler(req).await.into_response()
}

#[cfg(feature = "ssr")]
async fn sse_handler(
  TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
  State(core): State<BitcoinCore>,
) -> Response {
  use crate::types::LiveEvent;

  tracing::debug!("`{}` connected", user_agent.as_str());

  let initial_block_count = core.get_block_count().await;

  // A `Stream` that repeats an event every second
  let stream = stream::once(async move {
    // Send initial value for block
    LiveEvent::BlockCount(initial_block_count)
  })
  .chain(EVENT_CHANNEL.clone())
  .map(|event| match event {
    // Note:
    // Any data sent over wire needs to be serialized into a stringified JSON using `serde_json::to_string` (similar to `JSON.stringify` in JS)
    // It will be deserialized at frontend using `serde_json::from_str` (similar to `JSON.parse` in JS) - see implementation in `App.rs -> App`
    LiveEvent::NewInscription(data) | LiveEvent::RandomInscription(data) => {
      let s = serde_json::to_string(&data).unwrap();
      // type anotation once. was infered from function signature before -> Sse<impl Stream<Item = Result<Event, Infallible>>>
      let event: Result<Event, Infallible> = Ok(Event::default().event("inscription").data(s));
      event
    }
    LiveEvent::MempoolInfo(data) => {
      let s = serde_json::to_string(&data).unwrap();
      Ok(Event::default().event("info").data(s))
    }
    LiveEvent::BlockCount(data) => {
      let s = serde_json::to_string(&data).unwrap();
      Ok(Event::default().event("block").data(s))
    }
  });

  let sse = Sse::new(stream).keep_alive(
    axum::response::sse::KeepAlive::new()
      .interval(Duration::from_secs(1))
      .text("keep-alive-text"),
  );

  // additional header is required to signal to nginx to disable buffering/caching
  // see also https://serverfault.com/questions/801628/
  ([("X-Accel-Buffering", "no")], sse).into_response()
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  // load the .env file located in CWD or its parents in sequence.
  dotenv::dotenv().ok();

  let conf = get_configuration(None).await.unwrap();
  let leptos_site_addr = conf.leptos_options.site_addr;

  let mut console_addr = leptos_site_addr;
  console_addr.set_port(leptos_site_addr.port().checked_add(6).unwrap());

  let console_layer = console_subscriber::ConsoleLayer::builder()
    .retention(std::time::Duration::from_secs(60))
    .server_addr(console_addr)
    .build()
    .0;

  tracing_subscriber::registry()
    .with(console_layer)
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env()
        //.unwrap_or_else(|_| "tokio=debug,runtime=trace".into()),
        .unwrap_or_else(|_| "info".into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

  tracing::info!("spawning console_subscriber at {}", console_addr);

  let mut metrics_addr = leptos_site_addr;
  metrics_addr.set_port(leptos_site_addr.port().checked_add(9).unwrap());

  tracing::info!("spawning process_metrics at {}", metrics_addr);
  let task_process_metrics = tokio::task::Builder::new()
    .name("process_metrics")
    .spawn(spawn_process_metrics(metrics_addr))
    .unwrap();

  let task_inscription_ticks = tokio::task::Builder::new()
    .name("inscription_ticks")
    .spawn(spawn_inscription_ticks())
    .unwrap();

  let task_blockinfo_ticks = tokio::task::Builder::new()
    .name("blockinfo_ticks")
    .spawn(spawn_blockinfo_ticks())
    .unwrap();

  spawn_app().await;

  let result = tokio::try_join! {
      task_inscription_ticks,
      task_blockinfo_ticks,
      task_process_metrics,
  };
  result?;

  Ok(())
}

#[cfg(feature = "ssr")]
// #[tracing::instrument]
async fn spawn_app() {
  use axum_otel_metrics::HttpMetricsLayerBuilder;
  use tower_http::services::ServeDir;
  let metrics = HttpMetricsLayerBuilder::new().build();

  // Setting this to None means we'll be using cargo-leptos and its env vars.
  // when not using cargo-leptos None must be replaced with Some("Cargo.toml")
  let conf = get_configuration(None).await.unwrap();
  let leptos_options = conf.leptos_options;
  let addr = &leptos_options.site_addr;
  let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

  let state = AppState {
    core: backend::BitcoinCore::new(),
    _db: Database {},
    _client: HttpClient {},
  };

  // -- extern crate axum-prometheus: currently disabled, because it can't be used in conjunction with process_metrics
  // use axum_prometheus::PrometheusMetricLayer;
  // let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

  // build our application with a route
  let app = Router::new()
    .merge(metrics.routes())
    .route("/api/events", get(sse_handler))
    .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
    .route("/special/:id", get(custom_handler))
    .route("/preview/:inscription_id", get(server_actions::preview))
    .route("/content/:inscription_id", get(server_actions::content))
    .with_state(state)
    // related: axum-prometheus
    //.route("/metrics", get(|| async move { metric_handle.render() }))
    .leptos_routes(&leptos_options, routes, |cx| view! { cx, <App/> })
    // punks_*.webp fallback
    .nest_service("/punks", ServeDir::new("./tmp/punks"))
    .fallback(fallback::file_and_error_handler)
    .with_state(leptos_options.clone())
    .layer(metrics)
    .layer(TraceLayer::new_for_http())
    .merge(Router::new());

  tracing::info!("spawning app at {}", &addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

#[cfg(feature = "ssr")]
// #[tracing::instrument]
pub async fn spawn_inscription_ticks() {
  let mut ordipool: HashMap<String, Option<Inscription>> = HashMap::new();

  let backend = backend::BitcoinCore::new();

  // todo: print more relevant config stuff
  tracing::info!(?backend);

  let mut interval = tokio::time::interval(std::time::Duration::from_millis(3142));
  loop {
    interval.tick().await;
    server_actions::tick_inscriptions(&backend, &mut ordipool).await;
  }
}

#[cfg(feature = "ssr")]
// #[tracing::instrument]
pub async fn spawn_blockinfo_ticks() {
  let backend = backend::BitcoinCore::new();

  let mut interval = tokio::time::interval(std::time::Duration::from_millis(5000));

  loop {
    interval.tick().await;
    server_actions::tick_blockinfo(&backend).await;
  }
}

#[cfg(feature = "ssr")]
// #[tracing::instrument]
pub async fn spawn_process_metrics(addr: std::net::SocketAddr) {
  use metrics_exporter_prometheus::PrometheusBuilder;
  use metrics_process::Collector;

  let builder = PrometheusBuilder::new();
  let handle = builder
    .install_recorder()
    .expect("failed to install Prometheus recorder");

  let collector = Collector::default();
  // Call `describe()` method to register help string.
  collector.describe();

  let app = Router::new().route(
    "/metrics",
    get(move || {
      // Collect information just before handle '/metrics'
      collector.collect();
      std::future::ready(handle.render())
    }),
  );
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
  // isomorphic counters cannot work in a Client-Side-Rendered only
  // app as a server is required to maintain state
}

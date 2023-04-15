use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::app::*;
    use leptos::*;
    mod backend;
    use backend::Backend;
    extern crate ord_mini;
    use ord_mini::Inscription;
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
    use futures::stream::{self, Stream};
    use std::{convert::Infallible, time::Duration};
    use tokio_stream::StreamExt as _;
    use tower_http::trace::TraceLayer;
    use tracing_subscriber::prelude::*;
    use axum_prometheus::PrometheusMetricLayer;



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
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    tracing::debug!("`{}` connected", user_agent.as_str());

    // A `Stream` that repeats an event every second
    let stream = stream::once(async { LiveEvent::MempoolInfo("".to_owned()) })
        .chain(EVENT_CHANNEL.clone())
        .map(|event| match event {
            LiveEvent::NewInscription(data) | LiveEvent::RandomInscription(data) => {
                Ok(Event::default().event("inscription").data(data.as_str()))
            }

            LiveEvent::MempoolInfo(data) => Ok(Event::default().event("info").data(data.as_str())),
        });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let console_layer = console_subscriber::spawn();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_sse=debug,tower_http=debug".into()),
        )
        .with(console_layer)
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    crate::app::register_server_functions();

    let mut ordipool: HashMap<String, Option<Inscription>> = HashMap::new();
    let backend = std::env::var("BACKEND")
        .unwrap_or("bitcoin_core".to_string())
        .to_lowercase();
    //let backend_str = backend.as_str();
    let backend_space = backend::Space::new();
    let backend_bitcoin_core = backend::BitcoinCore::new();

    // todo print more relevant config stuff
    dbg!(&backend_bitcoin_core);
    // if std::env::var("RUST_BACKTRACE").is_ok() {
    //     let mut buffer = String::new();
    //     let stdin = std::io::stdin(); // We get `Stdin` here.
    //     println!("Press enter to start with config above. Crtl+C to abort.");
    //     stdin.read_line(&mut buffer).expect(msg);
    // }

    tokio::spawn(async move {
        //let mut runs = 100u32;

        let mut interval = tokio::time::interval(std::time::Duration::from_millis(3142));
        loop {
            interval.tick().await;
            //log!("tick2");
            //runs += 1;
            //let punk = format!("punk_{}.webp", &runs);
            //INSCRIPTION_CHANNEL.send(&punk).await.unwrap();
            if backend == "space" {
                server_actions::tick_space(&backend_space, &mut ordipool).await;
            } else if backend == "bitcoin_core" {
                server_actions::tick_bitcoin_core(&backend_bitcoin_core, &mut ordipool).await;
            } else {
                panic!("Unknown backend");
            }
            //server_actions::tick(&backend, &mut ordipool).await;
            // do something
        }
    });

    // Setting this to None means we'll be using cargo-leptos and its env vars.
    // when not using cargo-leptos None must be replaced with Some("Cargo.toml")
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    let state = AppState {
        core: backend::BitcoinCore::new(),
        _db: Database {},
        _client: HttpClient {},
    };

    //tokio::spawn(async move { start_metrics_server() });

    // build our application with a route
    let app = Router::new()
        .route("/api/events", get(sse_handler))
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .route("/special/:id", get(custom_handler))
        .route("/preview/:inscription_id", get(server_actions::preview))
        .route("/content/:inscription_id", get(server_actions::content))
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .with_state(state)
        .leptos_routes(leptos_options.clone(), routes, |cx| view! { cx, <App/> })
        //todo punks_ fallback
        .fallback(fallback::file_and_error_handler)
        .layer(Extension(Arc::new(leptos_options.clone())))
        .layer(prometheus_layer)
        .layer(TraceLayer::new_for_http());

    tracing::debug!("listening on {}", leptos_options.site_addr);
    axum::Server::bind(&leptos_options.site_addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// #[cfg(feature = "ssr")]
// async fn start_metrics_server() {
//     //use tokio::net::unix::SocketAddr;
//      use std::future::ready;
//     use std::net::SocketAddr;

//     // setup_metrics_recorder
//     const EXPONENTIAL_SECONDS: &[f64] = &[
//         0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
//     ];

//     let recorder_handle = PrometheusBuilder::new()
//         .set_buckets_for_metric(
//             Matcher::Full("http_requests_duration_seconds".to_string()),
//             EXPONENTIAL_SECONDS,
//         )
//         .unwrap()
//         .install_recorder()
//         .unwrap();

//     let metrics_app = Router::new().route("/metrics", get(move || ready(recorder_handle.render())));

//     // NOTE: expose metrics endpoint on a different port
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3090));
//     tracing::debug!("listening on {}", addr);
//     axum::Server::bind(&addr)
//         .serve(metrics_app.into_make_service())
//         .await
//         .unwrap()
// }

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // isomorphic counters cannot work in a Client-Side-Rendered only
    // app as a server is required to maintain state
}

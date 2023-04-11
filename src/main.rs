use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use leptos::*;
    use crate::app::*;
    //mod server_actions;
    mod backend;
    use backend::Backend;
    extern crate ord_mini;
    use ord_mini::Inscription;
    use std::collections::HashMap;
    extern crate dotenv;
    extern crate num_format;
    extern crate leptos_axum;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    mod fallback;

    use axum::body::Body as AxumBody;
    use axum::{
        extract::{Extension, Path},
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
    use std::{convert::Infallible, net::SocketAddr, path::PathBuf, time::Duration};
    use tokio_stream::StreamExt as _;
    use tower_http::{services::ServeDir, trace::TraceLayer};
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    use std::sync::Arc;

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

// #[get("/style.css")]
// async fn css() -> impl Responder {
//     actix_files::NamedFile::open_async("./style/output.css").await
// }
#[cfg(feature = "ssr")]
async fn sse_handler(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    println!("`{}` connected", user_agent.as_str());

    // A `Stream` that repeats an event every second
    let stream = stream::repeat_with(|| Event::default().data("hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}

// #[get("/api/events")]
// async fn counter_events() -> impl Responder {
//     use futures::StreamExt;

//     let stream =
//         futures::stream::once(async { LiveEvent::MempoolInfo("".to_string()) })
//             .chain(EVENT_CHANNEL.clone())
//             .map(|event| {
//                 let string = match event {
//                     LiveEvent::NewInscription(value) |
//                     LiveEvent::RandomInscription(value) => {
//                         let value = value.as_str();
//                         format!(
//                             "event: inscription\ndata: {value}\n\n"
//                         )
//                     }

//                     LiveEvent::MempoolInfo(value) => {
//                         let value = value.as_str();
//                         format!(
//                             "event: info\ndata: {value}\n\n"
//                         )
//                     }
//                 };

//                 Ok(web::Bytes::from(string)) as Result<web::Bytes>
//             });
//     HttpResponse::Ok()
//         .insert_header(("Content-Type", "text/event-stream"))
//         .streaming(stream)
// }

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_sse=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("site");
    let static_files_service = ServeDir::new(assets_dir).append_index_html_on_directories(true);

    // crate::app::register_server_functions();

    // let app = Router::new()
    // .fallback_service(static_files_service)
    // .route("/sse", get(sse_handler))
    // .layer(TraceLayer::new_for_http());

    // // Setting this to None means we'll be using cargo-leptos and its env vars.
    // // when not using cargo-leptos None must be replaced with Some("Cargo.toml")

    // let conf = leptos::get_configuration(Some("Cargo.toml")).await.unwrap();

    // let addr = conf.leptos_options.site_addr.clone();
    // let routes = generate_route_list(|cx| view! { cx,
    //    <App/> });

    // let mut ordipool:  HashMap<String, Option<Inscription>> = HashMap::new();
    // let backend = std::env::var("BACKEND").unwrap_or("bitcoin_core".to_string()).to_lowercase();
    // //let backend_str = backend.as_str();
    // let backend_space = backend::Space::new();
    // let backend_bitcoin_core = backend::BitcoinCore::new();

    // // todo print more relevant config stuff
    // dbg!(&backend_bitcoin_core);
    // if std::env::var("RUST_BACKTRACE").is_ok() {
    //     let mut buffer = String::new();
    //     let stdin = std::io::stdin(); // We get `Stdin` here.
    //     println!("Press enter to start with config above. Crtl+C to abort.");
    //     stdin.read_line(&mut buffer)?;
    // }

    // actix_rt::spawn(async move {
    //     //let mut runs = 100u32;
    //     let mut interval = actix_rt::time::interval(std::time::Duration::from_millis(3142));
    //     loop {
    //         interval.tick().await;
    //         //log!("tick2");
    //         //runs += 1;
    //         //let punk = format!("punk_{}.webp", &runs);
    //         //INSCRIPTION_CHANNEL.send(&punk).await.unwrap();
    //         if backend == "space" {
    //             server_actions::tick_space(&backend_space, &mut ordipool).await;
    //         } else if backend == "bitcoin_core" {
    //             server_actions::tick_bitcoin_core(&backend_bitcoin_core, &mut ordipool).await;
    //         } else {
    //             panic!("Unknown backend");
    //         }
    //         //server_actions::tick(&backend, &mut ordipool).await;
    //         // do something
    //     }
    // });

    // build our application with a route
    let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr.clone();
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .route("/special/:id", get(custom_handler))
        .route("/preview/:inscription_id", get(server_actions::preview))
        .route("/content/:inscription_id", get(server_actions::content))
        .route("/sse", get(sse_handler))
        .leptos_routes(leptos_options.clone(), routes, |cx| view! { cx, <App/> })
        .fallback_service(static_files_service)
        .layer(TraceLayer::new_for_http());

    // run it
    // HttpServer::new(move || {
    //     let leptos_options = &conf.leptos_options;
    //     let site_root = &leptos_options.site_root;

    //     App::new()
    //         .service(counter_events)
    //         .service(web::rcargoesource("/preview/{inscription_id}").to(server_actions::preview))
    //         .service(web::resource("/content/{inscription_id}").to(server_actions::content))
    //         .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
    //         .leptos_routes(leptos_options.to_owned(), routes.to_owned(), |cx| view! { cx, <App/> })
    //         .service(Files::new("/punks/", "/tmp/punks"))
    //         .service(Files::new("/", &site_root))
    //         //.wrap(middleware::Compress::default())
    // })
    // .bind(&addr)?
    // .run()
    // .await

    // let addr = "[::]:3000".parse().unwrap();
    let addr = "[::]:3000".parse::<std::net::SocketAddr>().unwrap();

    tracing::debug!("listening on {}", addr);
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

use cfg_if::cfg_if;
mod app;

// boilerplate to run in different modes
cfg_if! {

    // server-only stuff
    if #[cfg(feature = "ssr")] {
        use leptos::*;
        use actix_files::{Files};
        use actix_web::*;
        use crate::app::*;
        use leptos_actix::{generate_route_list, LeptosRoutes};
        mod server_actions;
        mod backend;
        use backend::Backend;
        extern crate ord_mini;
        use ord_mini::Inscription;
        use std::collections::HashMap;
        extern crate dotenv;
        extern crate num_format;


        #[get("/style.css")]
        async fn css() -> impl Responder {
            actix_files::NamedFile::open_async("./style/output.css").await
        }

        #[get("/api/events")]
        async fn counter_events() -> impl Responder {
            use futures::StreamExt;

            let stream =
                futures::stream::once(async { LiveEvent::MempoolInfo("".to_string()) })
                    .chain(EVENT_CHANNEL.clone())
                    .map(|event| {
                        let string = match event {
                            LiveEvent::NewInscription(value) |
                            LiveEvent::RandomInscription(value) => {
                                let value = value.as_str();
                                format!(
                                    "event: inscription\ndata: {value}\n\n"
                                )
                            }

                            LiveEvent::MempoolInfo(value) => {
                                let value = value.as_str();
                                format!(
                                    "event: info\ndata: {value}\n\n"
                                )
                            }
                        };

                        Ok(web::Bytes::from(string)) as Result<web::Bytes>
                    });
            HttpResponse::Ok()
                .insert_header(("Content-Type", "text/event-stream"))
                .streaming(stream)
        }


        #[actix_web::main]
        async fn main() -> std::io::Result<()> {
            dotenv::dotenv().ok();

            crate::app::register_server_functions();

            // Setting this to None means we'll be using cargo-leptos and its env vars.
            // when not using cargo-leptos None must be replaced with Some("Cargo.toml")

            let conf = leptos::get_configuration(Some("Cargo.toml")).await.unwrap();

            let addr = conf.leptos_options.site_addr.clone();
            let routes = generate_route_list(|cx| view! { cx,
               <App/> });

            let mut ordipool:  HashMap<String, Option<Inscription>> = HashMap::new();
            let backend = std::env::var("BACKEND").unwrap_or("bitcoin_core".to_string()).to_lowercase();
            //let backend_str = backend.as_str();
            let backend_space = backend::Space::new();
            let backend_bitcoin_core = backend::BitcoinCore::new();

            // todo print more relevant config stuff
            dbg!(&backend_bitcoin_core);
            if std::env::var("RUST_BACKTRACE").is_ok() {
                let mut buffer = String::new();
                let stdin = std::io::stdin(); // We get `Stdin` here.
                println!("Press enter to start with config above. Crtl+C to abort.");
                stdin.read_line(&mut buffer)?;
            }


            actix_rt::spawn(async move {
                //let mut runs = 100u32;
                let mut interval = actix_rt::time::interval(std::time::Duration::from_millis(3142));
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

            HttpServer::new(move || {
                let leptos_options = &conf.leptos_options;
                let site_root = &leptos_options.site_root;

                App::new()
                    .service(counter_events)
                    .service(web::resource("/preview/{inscription_id}").to(server_actions::preview))
                    .service(web::resource("/content/{inscription_id}").to(server_actions::content))
                    .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
                    .leptos_routes(leptos_options.to_owned(), routes.to_owned(), |cx| view! { cx, <App/> })
                    .service(Files::new("/punks/", "/tmp/punks"))
                    .service(Files::new("/", &site_root))
                    //.wrap(middleware::Compress::default())
            })
            .bind(&addr)?
            .run()
            .await
        }


    }

    // client-only main for Trunk
    else {
        pub fn main() {
            // isomorphic counters cannot work in a Client-Side-Rendered only
            // app as a server is required to maintain state
        }

    }
}

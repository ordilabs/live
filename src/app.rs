use crate::app::components::*;
use crate::app::providers::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod providers;

#[cfg(feature = "ssr")]
use std::sync::atomic::{AtomicI32, Ordering};

#[cfg(feature = "ssr")]
use broadcaster::BroadcastChannel;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    _ = GetServerCount::register();
    _ = AdjustServerCount::register();
    _ = ClearServerCount::register();
    _ = GetLastInscription::register();
    _ = SetDarkTheme::register();
}

#[cfg(feature = "ssr")]
static COUNT: AtomicI32 = AtomicI32::new(0);

#[cfg(feature = "ssr")]
#[derive(Clone)]
pub enum LiveEvent {
    NewInscription(String),
    RandomInscription(String),
    MempoolInfo(String),
}

#[cfg(feature = "ssr")]
lazy_static::lazy_static! {
    pub static ref COUNT_CHANNEL: BroadcastChannel<i32> = BroadcastChannel::new();
    pub static ref INSCRIPTION_CHANNEL: BroadcastChannel<String> = BroadcastChannel::new();
    pub static ref EVENT_CHANNEL: BroadcastChannel<LiveEvent> = BroadcastChannel::new();
}
// "/api" is an optional prefix that allows you to locate server functions wherever you'd like on the server
#[server(GetServerCount, "/api")]
pub async fn get_server_count() -> Result<i32, ServerFnError> {
    Ok(COUNT.load(Ordering::Relaxed))
}

#[server(GetLastInscription, "/api")]
pub async fn get_last_inscription() -> Result<String, ServerFnError> {
    Ok("".to_string())
}

#[server(AdjustServerCount, "/api")]
pub async fn adjust_server_count(delta: i32, msg: String) -> Result<i32, ServerFnError> {
    let new = COUNT.load(Ordering::Relaxed) + delta;
    COUNT.store(new, Ordering::Relaxed);
    _ = COUNT_CHANNEL.send(&new).await;
    println!("message = {:?}", msg);
    Ok(new)
}

#[server(ClearServerCount, "/api")]
pub async fn clear_server_count() -> Result<i32, ServerFnError> {
    COUNT.store(0, Ordering::Relaxed);
    _ = COUNT_CHANNEL.send(&0).await;
    Ok(0)
}

#[server(SetDarkTheme, "/api")]
pub async fn set_dark_theme(cx: Scope, is_dark: bool) -> Result<bool, ServerFnError> {
    use axum::http::header::{HeaderMap, HeaderValue, SET_COOKIE};
    use leptos_axum::{ResponseOptions, ResponseParts};

    let response =
        use_context::<ResponseOptions>(cx).expect("to have leptos_axum::ResponseOptions provided");
    let mut response_parts = ResponseParts::default();
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(&format!("darkmode={is_dark}; Path=/")).expect("to create header value"),
    );
    response_parts.headers = headers;

    response.overwrite(response_parts);
    Ok(is_dark)
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    provide_theme_context(cx);

    #[cfg(not(feature = "ssr"))]
    let (multiplayer_value, info_value) = {
        use futures::StreamExt;

        let mut source = gloo_net::eventsource::futures::EventSource::new("/api/events")
            .expect("couldn't connect to SSE stream");
        let s = create_signal_from_stream(
            cx,
            source.subscribe("inscription").unwrap().map(|value| {
                value
                    .expect("no message event")
                    .1
                    .data()
                    .as_string()
                    .expect("expected string value")
            }),
        );

        let s2 = create_signal_from_stream(
            cx,
            source.subscribe("info").unwrap().map(|value| {
                value
                    .expect("no message event")
                    .1
                    .data()
                    .as_string()
                    .expect("expected string value")
            }),
        );

        on_cleanup(cx, move || source.close());
        (s, s2)
    };

    #[cfg(feature = "ssr")]
    let ((multiplayer_value, _), (info_value, _)) = (
        create_signal(cx, None::<String>),
        create_signal(cx, None::<String>),
    );

    let initial_items: Vec<_> = (0..6).map(|n| format!("punk_{}.webp", n)).collect();

    let theme_ctx =
        use_context::<ThemeContext>(cx).expect("Failed to get ThemeContext");

    view! {
        cx,
        <Router>
          <Meta name="twitter:card" content="summary_large_image"/>
          <Meta name="twitter:site" content="@Ordilabs_org"/>
          <Meta name="twitter:title" content="Ordinals Live"/>
          <Meta name="twitter:description" content="Ordinals mempool viewer. View inscriptions before they're inscribed!"/>
          <Meta name="twitter:image" content="https://live.ordilabs.org/ordilabs-logo-name-white.png"/>
          <Link rel="shortcut icon" href="/favicon.png"/>
          <Stylesheet id="leptos" href="/pkg/ordilabs_live.css"/>
          <body class:dark=move || theme_ctx.is_dark.get()>
            <div class="flex h-full bg-white dark:bg-slate-800">
              <div class="flex flex-1 flex-col overflow-hidden">
                <Header />
                <div class="flex flex-1 items-stretch overflow-hidden">
                    <main class="flex-1 overflow-y-auto">
                      <div class="mx-auto max-w-7xl px-4 pt-8 sm:px-6 lg:px-8">
                        <div class="flex">
                          <h1 class="flex-1 text-2xl font-bold text-gray-900 dark:text-gray-100">
                              "Live unconfirmed inscriptions"
                          </h1>
                        </div>
                        <div class="text-xs text-gray-900 dark:text-gray-100">{info_value}</div>
                        <LiveGrid initial_items multiplayer_value/>
                      </div>
                    </main>
                </div>
                <Footer />
              </div>
            </div>
          </body>
        </Router>
    }
}

use crate::app::components::*;
// use crate::app::pages::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod pages;

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
}

#[cfg(feature = "ssr")]
static COUNT: AtomicI32 = AtomicI32::new(0);

#[cfg(feature = "ssr")]
lazy_static::lazy_static! {
    pub static ref COUNT_CHANNEL: BroadcastChannel<i32> = BroadcastChannel::new();
    pub static ref INSCRIPTION_CHANNEL: BroadcastChannel<String> = BroadcastChannel::new();
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

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    // setup_interval() only on client
    // #[cfg(not(feature = "ssr"))]
    // set_interval(|| log!("interval"), std::time::Duration::from_secs(1)).unwrap();

    #[cfg(not(feature = "ssr"))]
    let multiplayer_value = {
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

        on_cleanup(cx, move || source.close());
        s
    };

    #[cfg(feature = "ssr")]
    let (multiplayer_value, _) = create_signal(cx, None::<String>);

    let initial_items = vec![
        "punk_0.webp".to_string(),
        "punk_1.webp".to_string(),
        "punk_2.webp".to_string(),
        "punk_3.webp".to_string(),
    ];

    view! {
        cx,
        <Body class="bg-white dark:bg-slate-800" />
        <Router>
        <Meta name="twitter:card" content="summary_large_image"/>
        <Meta name="twitter:site" content="@OrdilabsOrg"/>
        <Meta name="twitter:title" content="Ordinals Live"/>
        <Meta name="twitter:description" content="Ordinals mempool viewer. View inscriptions before they're inscribed!"/>
        <Meta name="twitter:image" content="https://live.ordilabs.org/ordilabs-logo-name-white.png"/>
        <Link rel="shortcut icon" href="/favicon.png"/>
        <Stylesheet id="leptos" href="/pkg/ordilabs_live.css"/>

        <div class="flex h-full">
        // <!-- Content area -->
        <div class="flex flex-1 flex-col overflow-hidden">
           <Header />

           <span style="display: none">"Multiplayer Value: " {move || multiplayer_value.get().unwrap_or_default().to_string()}</span>

            // <!-- Main content -->
            <div class="flex flex-1 items-stretch overflow-hidden">
                <main class="flex-1 overflow-y-auto">
                    <div class="mx-auto max-w-7xl px-4 pt-8 sm:px-6 lg:px-8">
                        <div class="flex">
                            <h1 class="flex-1 text-2xl font-bold text-gray-900 dark:text-gray-100">"Stream"</h1>
                            <div class="ml-6 flex items-center rounded-lg bg-gray-100 p-0.5 sm:hidden">
                                <button type="button"
                                    class="rounded-md p-1.5 text-gray-400 hover:bg-white hover:shadow-sm focus:outline-none focus:ring-2 focus:ring-inset focus:ring-red-500">
                                    <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                        <path fill-rule="evenodd"
                                            d="M2 3.75A.75.75 0 012.75 3h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 3.75zm0 4.167a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75a.75.75 0 01-.75-.75zm0 4.166a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75a.75.75 0 01-.75-.75zm0 4.167a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75a.75.75 0 01-.75-.75z"
                                            clip-rule="evenodd" />
                                    </svg>
                                    <span class="sr-only">"Use list view"</span>
                                </button>
                                <button type="button"
                                    class="ml-0.5 rounded-md bg-white p-1.5 text-gray-400 shadow-sm focus:outline-none focus:ring-2 focus:ring-inset focus:ring-red-500">
                                    <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                        <path fill-rule="evenodd"
                                            d="M4.25 2A2.25 2.25 0 002 4.25v2.5A2.25 2.25 0 004.25 9h2.5A2.25 2.25 0 009 6.75v-2.5A2.25 2.25 0 006.75 2h-2.5zm0 9A2.25 2.25 0 002 13.25v2.5A2.25 2.25 0 004.25 18h2.5A2.25 2.25 0 009 15.75v-2.5A2.25 2.25 0 006.75 11h-2.5zm9-9A2.25 2.25 0 0011 4.25v2.5A2.25 2.25 0 0013.25 9h2.5A2.25 2.25 0 0018 6.75v-2.5A2.25 2.25 0 0015.75 2h-2.5zm0 9A2.25 2.25 0 0011 13.25v2.5A2.25 2.25 0 0013.25 18h2.5A2.25 2.25 0 0018 15.75v-2.5A2.25 2.25 0 0015.75 11h-2.5z"
                                            clip-rule="evenodd" />
                                    </svg>
                                    <span class="sr-only">"Use grid view"</span>
                                </button>
                            </div>
                        </div>

                        // <!-- Tabs -->
                        <TypeTabs />
                        // <!-- Gallery -->
                        <LiveGrid initial_items multiplayer_value/>
                    </div>
                </main>
            </div>
            <Footer />
        </div>
    </div>
        </Router>
    }
}

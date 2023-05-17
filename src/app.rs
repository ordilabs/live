use crate::app::components::*;
use crate::app::providers::*;
use crate::app::routes::*;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
pub mod functions;
#[macro_use]
pub mod i18n;
mod providers;
mod routes;

#[cfg(feature = "ssr")]
use crate::types::LiveEvent;

#[cfg(feature = "ssr")]
use broadcaster::BroadcastChannel;

#[cfg(feature = "ssr")]
lazy_static::lazy_static! {
    pub static ref INSCRIPTION_CHANNEL: BroadcastChannel<String> = BroadcastChannel::new();
    pub static ref EVENT_CHANNEL: BroadcastChannel<LiveEvent> = BroadcastChannel::new();
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
  provide_meta_context(cx);
  provide_theme_context(cx);
  provide_stream_context(cx);
  provide_i18n_context(cx);

  let theme_ctx = expect_context::<ThemeContext>(cx);

  view! { cx,
    <Router>
      <Body class=move || {
          let theme = match theme_ctx.is_dark.get() {
              true => "dark",
              false => "",
          };
          format!("h-screen {theme}")
      }/>
      <Meta name="twitter:card" content="summary_large_image"/>
      <Meta name="twitter:site" content="@Ordilabs_org"/>
      <Meta name="twitter:title" content="Ordinals Live"/>
      <Meta
        name="twitter:description"
        content="Ordinals mempool viewer. View inscriptions before they're inscribed!"
      />
      <Meta name="twitter:image" content="https://live.ordilabs.org/ordilabs-logo-name-white.png"/>
      <Link rel="shortcut icon" href="/favicon.png"/>
      <Stylesheet id="leptos" href="/pkg/ordilabs_live.css"/>
      <div class="flex h-full flex-1 flex-col overflow-hidden bg-white dark:bg-slate-800">
        <Header/>
        <div class="flex flex-1 items-stretch overflow-hidden">
          <main class="flex-1 overflow-y-auto">
            <Routes>
              <Route
                path="/"
                view=|cx| {
                    view! { cx, <Home/> }
                }
              />
              <Route
                path="inscription/:id"
                view=|cx| {
                    view! { cx, <Inscription/> }
                }
              />
            </Routes>
          </main>
        </div>
        <Footer/>
      </div>
    </Router>
  }
}

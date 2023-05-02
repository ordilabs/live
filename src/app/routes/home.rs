use crate::app::components::*;
use crate::app::providers::*;
use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
  let StreamContext {
    info, inscription, ..
  } = use_context::<StreamContext>(cx).expect("Failed to get StreamContext");

  let initial_inscriptions: Vec<_> = (0..6).map(|n| format!("punk_{}.webp", n)).collect();

  view! { cx,
    <div class="content py-8">
      <div class="flex">
        <h1 class="flex-1 text-2xl font-bold text-gray-900 dark:text-gray-100">
          "Live unconfirmed inscriptions"
        </h1>
      </div>
      // Keep heihgt for empty content - `&#8203;` doesn't work in Leptos - that's why adding a * and make it transparent
      // Question in Leptos Discord https://discord.com/channels/1031524867910148188/1102915498817097821/1102915498817097821
      <div class="text-xs text-gray-900 leading-4 dark:text-gray-100 empty:after:content-['*'] empty:after:opacity-0">{info}</div>
      <LiveGrid initial_inscriptions inscription_id=inscription/>
    </div>
  }
}

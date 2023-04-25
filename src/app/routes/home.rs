use crate::app::components::*;
use crate::app::providers::*;
use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
  let StreamContext { info, .. } =
    use_context::<StreamContext>(cx).expect("Failed to get StreamContext");

  let initial_inscriptions: Vec<_> = (0..6).map(|n| format!("punk_{}.webp", n)).collect();

  let inscription_info = info;

  view! { cx,
    <div class="mx-auto max-w-7xl px-4 pt-8 sm:px-6 lg:px-8">
      <div class="flex">
        <h1 class="flex-1 text-2xl font-bold text-gray-900 dark:text-gray-100">
          "Live unconfirmed inscriptions"
        </h1>
      </div>
      <div class="text-xs text-gray-900 dark:text-gray-100">{inscription_info}</div>
      <LiveGrid initial_inscriptions inscription_info/>
    </div>
  }
}

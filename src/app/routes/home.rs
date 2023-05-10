use crate::app::components::*;
use crate::app::i18n::T;
use crate::app::providers::*;
use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
  let StreamContext {
    info, inscription, ..
  } = use_context::<StreamContext>(cx).expect("Failed to get StreamContext");
  let i18n = use_context::<I18nContext>(cx).expect("Failed to get I18nContext");

  let initial_inscriptions: Vec<_> = (0..6).map(|n| format!("punk_{}.webp", n)).collect();

  view! { cx,
    <div class="content py-8">
      <div class="flex">
        <h1 class="flex items-center text-2xl font-bold text-gray-900 dark:text-gray-100
        before:mr-2 before:block before:w-6 before:h-6 before:border-4 before:rounded-full before:border-red-500 mr-2
        ">{i18n.t(cx, T::HomeTitle)}</h1>
      </div>
      <div class="text-xs text-gray-900 dark:text-gray-100 empty:after:content-['\u{200b}'] empty:after:inline-block">
        {info}
      </div>
      <LiveGrid initial_inscriptions inscription_id=inscription/>
    </div>
  }
}

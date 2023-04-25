use leptos::*;
use leptos_router::*;

#[component]
pub fn Inscription(cx: Scope) -> impl IntoView {
  let params = use_params_map(cx);
  let id = move || params().get("id").cloned();

  view! { cx,
    <div class="">
      <h1>"Inscription" {id()}</h1>
      <A href="/">
        <strong>"Back to home"</strong>
      </A>
    </div>
  }
}

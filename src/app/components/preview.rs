use leptos::*;

#[component]
pub fn Preview(cx: Scope, id: String, #[prop(optional)] class: String) -> impl IntoView {
  let preview_url = move || format!("/preview/{}", id);
  let clazz = format!("block w-full overflow-hidden bg-gray-100 {}", class);

  view! { cx,
    <div class=clazz>
      <iframe
        src=preview_url
        sandbox="allow-scripts"
        scrolling="no"
        loading="lazy"
        class="pointer-events-none object-cover"
      ></iframe>
    </div>
  }
}

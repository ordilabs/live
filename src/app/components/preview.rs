use leptos::*;

#[component]
pub fn Preview(cx: Scope, id: String, #[prop(optional)] class: String) -> impl IntoView {
  let preview_url = move || format!("/preview/{}", id);
  let clazz = format!(
    "ring-2 ring-red-500 group block w-full overflow-hidden rounded-lg bg-gray-100 {}",
    class
  );

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

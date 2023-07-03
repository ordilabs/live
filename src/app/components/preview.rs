use leptos::*;

#[component]
pub fn Preview(
  cx: Scope,
  hash: Signal<String>,
  #[prop(optional)] class: &'static str,
) -> impl IntoView {
  let preview_url = move || format!("/preview/{}", hash.get());

  view! { cx,
    <div class=format!("block w-full overflow-hidden bg-gray-100 {class}")>
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

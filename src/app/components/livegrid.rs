use crate::app::components::preview::*;
use leptos::*;
use leptos_router::*;
use log::info;

#[allow(dead_code)]
type Items = Vec<String>;

#[allow(clippy::used_underscore_binding)]
#[component]
pub fn LiveGrid(
  cx: Scope,
  initial_inscriptions: Vec<String>,
  inscription_id: ReadSignal<Option<String>>,
) -> impl IntoView {
  // let (next_counter_id, set_next_counter_id) = create_signal(cx, 0);
  let items = create_rw_signal::<Items>(cx, initial_inscriptions.clone());
  //provide_context(cx, CounterUpdater { set_counters });

  create_effect(cx, move |_| {
    let s = inscription_id();
    info!(
      "It works! {}",
      match &s {
        None => "'none'",
        Some(a) => a,
      }
    );
    if s.is_none() || s == Some("".to_string()) {
      return;
    }

    let id = s.unwrap();

    // if !counters().contains(&id) {
    items.update(|counters| {
      let _ = counters.pop();
      counters.insert(0, id)
    });
    // }
  });

  view! { cx,
    <section class="pt-8 pb-8" aria-labelledby="gallery-heading">
      <h2 id="gallery-heading" class="sr-only">
        "Recently viewed"
      </h2>
      <ul
        role="list"
        class="grid grid-cols-2 gap-x-4 gap-y-8 sm:grid-cols-3 sm:gap-x-6 md:grid-cols-4 lg:grid-cols-3 xl:grid-cols-4 xl:gap-x-8"
      >
        <For
          each=items
          key=|counter| counter.to_owned()
          view=move |cx, inscription_id| {
              view! { cx, <InscriptionItem id=inscription_id/> }
          }
        />
      </ul>
    </section>
  }
}

#[component]
pub fn InscriptionItem(cx: Scope, id: String) -> impl IntoView {
  // let id = move || id();
  let detail_url = format!("/inscription/{}", &id);
  let class = "ring-2 ring-red-500 rounded-lg aspect-w-10 aspect-h-10";
  info!("detail_url {}", &detail_url);

  view! { cx,
    <li class="relative">
      <A href=detail_url>
        <Preview class id=id/>
      </A>
    </li>
  }
}

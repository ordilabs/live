use crate::app::components::preview::*;
use leptos::*;
use leptos_router::*;

#[allow(dead_code)]
type CounterHolder = Vec<(usize, (ReadSignal<String>, WriteSignal<String>))>;

#[allow(clippy::used_underscore_binding)]
#[component]
pub fn LiveGrid(
  cx: Scope,
  initial_inscriptions: Vec<String>,
  inscription_id: ReadSignal<Option<String>>,
) -> impl IntoView {
  let (next_counter_id, set_next_counter_id) = create_signal(cx, 0);
  let (counters, set_counters) = create_signal::<CounterHolder>(cx, vec![]);
  //provide_context(cx, CounterUpdater { set_counters });

  for item in initial_inscriptions {
    let id = next_counter_id();
    let sig = create_signal(cx, item);
    set_counters.update(move |counters| counters.push((id, sig)));
    set_next_counter_id.update(|id| *id += 1);
  }

  create_effect(cx, move |_| {
    let s = inscription_id();
    if s.is_none() || s == Some("".to_string()) {
      return;
    }
    set_next_counter_id.update(|id| *id += 1);
    set_counters.update(|counters| {
      let m = (next_counter_id() - 1) % counters.len();
      let (_id, (_read, write)) = counters.get(m).unwrap();
      write.set(s.unwrap().to_owned());
    });
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
          each=counters
          key=|counter| counter.0
          view=move |cx, (_, (inscription_id, _)): (usize, (ReadSignal<String>, WriteSignal<String>))| {
              view! { cx, <InscriptionItem id=inscription_id() /> }
          }
        />
      </ul>
    </section>
  }
}

#[component]
pub fn InscriptionItem(cx: Scope, id: String) -> impl IntoView {
  let detail_url = format!("/inscription/{}", &id);
  let class = "aspect-w-10 aspect-h-10".to_string();
  view! { cx,
    <li class="relative">
      <A href=detail_url>
        <Preview class id />
      </A>
    </li>
  }
}

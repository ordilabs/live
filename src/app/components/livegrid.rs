use leptos::*;
use leptos_router::*;

#[allow(dead_code)]
type CounterHolder = Vec<(usize, (ReadSignal<String>, WriteSignal<String>))>;

#[allow(clippy::used_underscore_binding)]
#[component]
pub fn LiveGrid(
  cx: Scope,
  initial_inscriptions: Vec<String>,
  inscription_info: ReadSignal<Option<String>>,
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
    let s = inscription_info();
    //log!("effect: multiplayer_value = {:?}", s);
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
    <section class="mt-8 pb-16" aria-labelledby="gallery-heading">
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
          view=move |cx, (id, (value, set_value)): (usize, (ReadSignal<String>, WriteSignal<String>))| {
              view! { cx, <InscriptionItem id value set_value/> }
          }
        />
      </ul>
    </section>
  }
}

#[component]
pub fn LiveItem(
  cx: Scope,
  id: usize,
  value: ReadSignal<String>,
  set_value: WriteSignal<String>,
) -> impl IntoView {
  //let CounterUpdater { set_counters } = use_context(cx).unwrap();
  //let input = move |_ev| set_value("hello".to_owned());

  //let default_value = move || value.get().to_owned() + " - " + id.to_string().as_ref();
  let _ = set_value;
  on_cleanup(cx, || log::debug!("deleted a row"));

  view! { cx, <li>{id} ": " {value}</li> }
}
#[component]
pub fn InscriptionItem(
  cx: Scope,
  id: usize,
  value: ReadSignal<String>,
  set_value: WriteSignal<String>,
) -> impl IntoView {
  let _ = set_value;
  let detail_url = move || format!("/inscription/{}", id);
  let preview_url = move || format!("/preview/{}", value.get());
  view! { cx,
    <li class="relative">
      <A href=detail_url>
        <div class="ring-2 ring-red-500 aspect-w-10 aspect-h-10 group block w-full overflow-hidden rounded-lg bg-gray-100">
          <iframe
            src=preview_url
            sandbox="allow-scripts"
            scrolling="no"
            loading="lazy"
            class="pointer-events-none object-cover"
          ></iframe>
        </div>
      </A>
    </li>
  }
}

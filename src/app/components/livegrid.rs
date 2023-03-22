use leptos::*;

// const MANY_COUNTERS: usize = 4;
#[allow(dead_code)]
type CounterHolder = Vec<(usize, (ReadSignal<String>, WriteSignal<String>))>;

// #[derive(Copy, Clone)]
// struct CounterUpdater {
//     set_counters: WriteSignal<CounterHolder>,
// }
#[allow(clippy::used_underscore_binding)]
#[component]
pub fn LiveGrid(
    cx: Scope,
    initial_items: Vec<String>,
    multiplayer_value: ReadSignal<Option<String>>,
) -> impl IntoView {
    let (next_counter_id, set_next_counter_id) = create_signal(cx, 0);
    let (counters, set_counters) = create_signal::<CounterHolder>(cx, vec![]);
    //provide_context(cx, CounterUpdater { set_counters });

    for item in initial_items {
        let id = next_counter_id();
        let sig = create_signal(cx, item);
        set_counters.update(move |counters| counters.push((id, sig)));
        set_next_counter_id.update(|id| *id += 1);
    }

    create_effect(cx, move |_| {
        let s = multiplayer_value();
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

    view! {cx,
        <section class="mt-8 pb-16" aria-labelledby="gallery-heading">
        <h2 id="gallery-heading" class="sr-only">"Recently viewed"</h2>
        <ul role="list"
            class="grid grid-cols-2 gap-x-4 gap-y-8 sm:grid-cols-3 sm:gap-x-6 md:grid-cols-4 lg:grid-cols-3 xl:grid-cols-4 xl:gap-x-8">

            <For
                    each=counters
                    key=|counter| counter.0
                    view=move |cx, (id, (value, set_value)): (usize, (ReadSignal<String>, WriteSignal<String>))| {
                        view! { cx,
                            <InscriptionItem id value set_value/>
                        }
                    }
            />
        </ul>
    </section> }
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

    view! { cx,
        <li>{id}": "{value}</li>
    }
}
#[component]
pub fn InscriptionItem(
    cx: Scope,
    id: usize,
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
) -> impl IntoView {
    let _ = set_value;
    let preview_url = move || format!("/preview/{}", value.get());
    view! {
      cx,
      <li class="relative">
          // <!-- Current: "ring-2 ring-red-500 ring-offset-2", Default: "focus-within:ring-2 focus-within:ring-red-500 focus-within:ring-offset-2 focus-within:ring-offset-gray-100" -->
          <a href=preview_url target=preview_url title="Click to view the unconfirmed inscription.">
          <div
              class="ring-2 ring-red-500 aspect-w-10 aspect-h-10 group block w-full overflow-hidden rounded-lg bg-gray-100">
              // <!-- Current: "", Default: "group-hover:opacity-75" -->
              <iframe src=preview_url sandbox="allow-scripts" scrolling="no" loading="lazy" class="pointer-events-none object-cover"></iframe>

              <button type="button" class="absolute inset-0 focus:outline-none">
                  <span class="sr-only">"View details for ordinal"</span>
              </button>
          </div>
          // <p class="mt-2 block truncate text-sm font-medium text-gray-900"><a href=preview_url target="_blank">"View transaction"</a></p>
          // <p class="pointer-events-none block text-sm font-medium text-gray-500 text-ellipsis">"Tx:" {move || value.get()}</p>
          </a>
      </li>
    }
}

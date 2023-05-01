use crate::app::components::preview::*;
use leptos::*;
use leptos_router::*;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct Item {
  pub key: usize,
  pub hash: RwSignal<String>,
}

#[allow(dead_code)]
impl Item {
  pub fn new(cx: Scope, key: usize, hash: String) -> Self {
    let hash = create_rw_signal(cx, hash);
    Self { key, hash }
  }
}

#[allow(dead_code)]
type Items = Vec<Item>;

#[allow(clippy::used_underscore_binding)]
#[component]
pub fn LiveGrid(
  cx: Scope,
  initial_inscriptions: Vec<String>,
  inscription_id: ReadSignal<Option<String>>,
) -> impl IntoView {
  let max_item_id = initial_inscriptions.len();
  let next_item_id = store_value(cx, max_item_id);

  let initial_items: Items = initial_inscriptions
    .into_iter()
    .enumerate()
    .map(|(index, hash)| Item::new(cx, index, hash))
    .collect();

  let items = store_value::<Items>(cx, initial_items);

  create_effect(cx, move |_| {
    let next = inscription_id();

    if next.is_none() || next == Some("".to_string()) {
      return;
    }

    let next_value = next.clone().unwrap();

    let item_id = next_item_id() % max_item_id;
    next_item_id.update_value(|id| *id += 1);

    items.update_value(|items| {
      let item = items.get(item_id).unwrap();
      item.hash.set(next_value);
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
          each=items
          key=|item| item.key
          view=move |cx, item: Item| {
              view! { cx,
                <li class="relative">
                  <A href=move || format!("/inscription/{}", item.hash.get())>
                    <Preview
                      class="ring-2 ring-red-500 rounded-lg aspect-w-10 aspect-h-10"
                      hash=item.hash.derive_signal(cx)
                    />
                  </A>
                </li>
              }
          }
        />
      </ul>
    </section>
  }
}

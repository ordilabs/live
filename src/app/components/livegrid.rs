use crate::app::components::preview::*;
use leptos::*;
use leptos_router::*;
use log::info;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Item {
  pub key: usize,
  pub hash: ReadSignal<String>,
  set_hash: WriteSignal<String>,
}

#[allow(dead_code)]
impl Item {
  pub fn new(cx: Scope, key: usize, hash: String) -> Self {
    let (hash, set_hash) = create_signal(cx, hash);
    Self {
      key,
      hash,
      set_hash,
    }
  }

  pub fn update(&self, hash: String) {
    self.set_hash.set(hash);
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

  let initial_items = initial_inscriptions
    .into_iter()
    .enumerate()
    .map(|(index, hash)| Item::new(cx, index, hash))
    .collect();

  let items = create_rw_signal::<Items>(cx, initial_items);

  create_effect(cx, move |_| {
    let next = inscription_id();
    info!(
      "It works! {}",
      match next.clone() {
        None => "none".to_string(),
        Some(a) => a.to_owned(),
      }
    );

    if next.is_none() || next == Some("".to_string()) {
      return;
    }

    let next_value = next.clone().unwrap();
    info!("next {}", &next_value);

    let item_id = next_item_id() % max_item_id;
    info!("item_id {}", &item_id);
    next_item_id.update_value(|id| *id += 1);
    info!("item_id {}", next_item_id());

    items.update(|items| {
      let item = items.get(item_id).unwrap();
      info!("update2 {}", &item.key);
      item.update(next_value.clone());
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
          key=|item| item.key.clone()
          view=move |cx, item| {
              let hash = move || item.hash.get();
              view! { cx,
                <li class="relative">
                  <p class="text-white">{&hash()}</p>
                  <p class="text-white">{format!("/inscription/{}", & hash())}</p>
                </li>
              }
          }
        />
      </ul>
    </section>
  }
}

#[component]
pub fn InscriptionItem(cx: Scope, id: Signal<String>) -> impl IntoView {
  let detail_url = move || format!("/inscription/{}", &id());
  info!("detail_url {}", &detail_url());
  let class = "ring-2 ring-red-500 rounded-lg aspect-w-10 aspect-h-10";

  view! { cx,
    <li class="relative">
      <A href=detail_url()>
        <Preview class id=id()/>
      </A>
    </li>
  }
}

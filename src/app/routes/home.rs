use crate::app::components::*;
use crate::app::i18n::T;
use crate::app::providers::*;
use crate::types::{compare_media, MempoolAllInfo};
use leptos::*;
use ord_labs::Media;
use ord_labs::Media::*;

#[derive(Eq, Hash, PartialEq, Clone)]
struct MempoolInfoData {
  pub count: usize,
  pub size: usize,
}

type MempoolInfoTotal = std::collections::HashMap<Media, MempoolInfoData>;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
  let StreamContext {
    info, inscription, ..
  } = expect_context::<StreamContext>(cx);

  let infos = create_memo::<MempoolAllInfo>(cx, move |_| {
    // get infos
    let mut info_list = info().unwrap_or(Vec::new());
    // sort infos by media
    info_list.sort_by(|a, b| compare_media(&a.media, &b.media));
    // before returning
    info_list
  });

  let (infos_map, set_infos_map) = create_signal::<MempoolInfoTotal>(
    cx,
    // initial data - all media info are "empty" (zero size, zero count) by default
    std::collections::HashMap::from([
      (Audio, MempoolInfoData { count: 0, size: 0 }),
      (Iframe, MempoolInfoData { count: 0, size: 0 }),
      (Image, MempoolInfoData { count: 0, size: 0 }),
      (Pdf, MempoolInfoData { count: 0, size: 0 }),
      (Text, MempoolInfoData { count: 0, size: 0 }),
      (Unknown, MempoolInfoData { count: 0, size: 0 }),
      (Video, MempoolInfoData { count: 0, size: 0 }),
    ]),
  );

  // Update media map with data coming from "info" stream
  create_effect(cx, move |_| {
    let infos = info().unwrap_or(Vec::new());
    let mut map = infos_map();

    // Update map with data coming from `infos` stream
    for info in &infos {
      map.insert(
        info.media,
        MempoolInfoData {
          count: info.count,
          size: info.size,
        },
      );
    }

    set_infos_map(map);
  });

  let initial_inscriptions: Vec<_> = (0..6).map(|n| format!("punk_{}.webp", n)).collect();

  fn get_icon(cx: Scope, m: &Media) -> impl IntoView {
    let class = "mr-2";
    match m {
      Media::Audio => view! { cx, <IconAudio class/> }.into_view(cx),
      Media::Pdf => view! { cx, <IconPdf class/> }.into_view(cx),
      Media::Video => view! { cx, <IconVideo class/> }.into_view(cx),
      Media::Iframe => view! { cx, <IconIframe class/> }.into_view(cx),
      Media::Text => view! { cx, <IconText class/> }.into_view(cx),
      Media::Unknown => view! { cx, <IconUnknown class/> }.into_view(cx),
      Media::Image => view! { cx, <IconImage class/> }.into_view(cx),
    }
  }

  fn get_label(m: &Media, count: usize) -> T {
    match m {
      Media::Audio => {
        if count > 1 {
          T::Audios
        } else {
          T::Audio
        }
      }
      Media::Pdf => {
        if count > 1 {
          T::Pdfs
        } else {
          T::Pdf
        }
      }
      Media::Video => {
        if count > 1 {
          T::Videos
        } else {
          T::Video
        }
      }
      Media::Iframe => {
        if count > 1 {
          T::Iframes
        } else {
          T::Iframe
        }
      }
      Media::Text => {
        if count > 1 {
          T::Texts
        } else {
          T::Text
        }
      }
      Media::Unknown => {
        if count > 1 {
          T::Unknowns
        } else {
          T::Unknown
        }
      }
      Media::Image => {
        if count > 1 {
          T::Images
        } else {
          T::Image
        }
      }
    }
  }

  view! { cx,
    <div class="content py-8">
      <div class="flex justify-center md:justify-start">
        <h1 class="flex items-center text-2xl font-bold text-gray-900 dark:text-gray-100
        before:mr-2 before:block before:w-6 before:h-6 before:border-4 before:rounded-full before:border-red-500 mr-2 mb-4
        ">{t!(cx, T::HomeTitle)}</h1>
      </div>
      <div class="text-base text-gray-600 dark:text-gray-100">
        <Show
          when=move || info().is_some()
          fallback=|cx| {
              view! { cx,
                <div class="pl-2 py-1">
                  <IconLoader/>
                </div>
              }
          }
        >
          <div class="flex flex-col md:flex-row items-center md:items-start justify-center md:justify-start empty:after:content-['\u{200b}'] empty:after:inline-block empty:after:h-6">
            <For
              each=infos
              key=|info| {
                  format! {
                      "{:?}-{}", & info.media, & info.count
                  }
              }
              view=move |cx, info| {
                  let icon = move || get_icon(cx, &info.media);
                  let count = move || format!("{}", & info.count);
                  let label = move || {
                      let l = get_label(&info.media, info.count);
                      t!(cx, l)
                  };
                  let size = move || format!("({:.1} kB)", info.size as f64 / 1024.0);
                  view! { cx,
                    <Show
                      when=move || { info.count > 0 }
                      fallback=|_| {
                          view! { cx, <></> }
                      }
                    >
                      <div class="flex items-center md:pr-6 md:pb-0 pb-2 last:pb-0">
                        {icon()} " " <span class="whitespace-nowrap">{count()} " " {label()} " " {size()}</span>
                      </div>
                    </Show>
                  }
              }
            />
          </div>
        </Show>
      </div>
      <LiveGrid initial_inscriptions inscription_id=inscription/>
    </div>
  }
}

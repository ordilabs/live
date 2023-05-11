use crate::app::components::*;
use crate::app::i18n::T;
use crate::app::providers::*;
use crate::types::MempoolInfo;
use leptos::*;
use ord_labs::Media;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
  let StreamContext {
    info, inscription, ..
  } = use_context::<StreamContext>(cx).expect("Failed to get StreamContext");
  let i18n = use_context::<I18nContext>(cx).expect("Failed to get I18nContext");

  let infos = move || info().unwrap_or(Vec::new());

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
      <div class="flex">
        <h1 class="flex items-center text-2xl font-bold text-gray-900 dark:text-gray-100
        before:mr-2 before:block before:w-6 before:h-6 before:border-4 before:rounded-full before:border-red-500 mr-2 mb-4
        ">{i18n.clone().t(cx, T::HomeTitle)}</h1>
      </div>
      <div class="flex flex-wrap text-base text-gray-600 dark:text-gray-100">
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
          <For
            each=move || { infos().into_iter().enumerate() }
            key=|i| *i
            view=move |cx, (_, MempoolInfo { media, count, size })| {
                view! { cx,
                  <div class="flex items-center pl-2 pr-4">
                    {get_icon(cx, &media)} " " {format!("{count}")} " "
                    {i18n.clone().t(cx, get_label(&media, count))} " "
                    {format!("({:.1} kB)", size as f64 / 1024.0)}
                  </div>
                }
            }
          />
        </Show>
      </div>
      <LiveGrid initial_inscriptions inscription_id=inscription/>
    </div>
  }
}

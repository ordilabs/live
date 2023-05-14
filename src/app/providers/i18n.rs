use leptos::*;

use crate::app::functions::SetLocale;
use crate::app::i18n::{I18nContext, Locale};

#[allow(dead_code)]
#[cfg(not(feature = "ssr"))]
fn initial_locale(_cx: Scope) -> Locale {
  use std::str::FromStr;
  use wasm_bindgen::JsCast;

  let doc = document().unchecked_into::<web_sys::HtmlDocument>();
  let cookies = doc.cookie().unwrap_or_default();

  for cookie in cookies.split("; ") {
    let trim_by = format!("locale=");
    if let Some(value) = cookie.strip_prefix(trim_by.as_str()) {
      return Locale::from_str(&value).unwrap_or_default();
    }
  }

  Locale::default()
}

#[cfg(feature = "ssr")]
fn initial_locale(cx: Scope) -> Locale {
  use axum_extra::extract::cookie::CookieJar;
  use std::str::FromStr;

  use_context::<leptos_axum::RequestParts>(cx)
    .and_then(|req| {
      let cookies = CookieJar::from_headers(&req.headers);
      return cookies
        .get("locale")
        .and_then(|c| Locale::from_str(&c.value()).ok());
    })
    .unwrap_or_default()
}

#[allow(dead_code)]
pub fn provide_i18n_context(cx: Scope) {
  if use_context::<I18nContext>(cx).is_none() {
    let initial = initial_locale(cx);
    let set_locale_action = create_server_action::<SetLocale>(cx);

    let input = set_locale_action.input();
    let value = set_locale_action.value();

    let locale = Signal::derive(cx, move || {
      match (input(), value()) {
        // use current input optimistically
        (Some(submission), _) => submission.locale,
        // or previous value confirmed by server
        (_, Some(Ok(value))) => value,
        // or initial value
        _ => initial,
      }
    });

    provide_context(cx, I18nContext::new(locale, set_locale_action));
  }
}

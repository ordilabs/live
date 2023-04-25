use leptos::*;
use std::*;

use crate::app::functions::theme::SetDarkTheme;

#[derive(Clone)]
pub(crate) struct ThemeContext {
  pub set_dark_action: Action<SetDarkTheme, Result<bool, ServerFnError>>,
  pub is_dark: Signal<bool>,
}

#[cfg(not(feature = "ssr"))]
fn initial_theme(_cx: Scope) -> bool {
  use wasm_bindgen::JsCast;

  let doc = document().unchecked_into::<web_sys::HtmlDocument>();
  let cookie = doc.cookie().unwrap_or_default();
  cookie.contains("darkmode=true")
}

#[cfg(feature = "ssr")]
fn initial_theme(cx: Scope) -> bool {
  use axum_extra::extract::cookie::CookieJar;

  use_context::<leptos_axum::RequestParts>(cx)
    .and_then(|req| {
      let cookies = CookieJar::from_headers(&req.headers);
      return cookies.get("darkmode").and_then(|v| match v.value() {
        "true" => Some(true),
        "false" => Some(false),
        _ => Some(false),
      });
    })
    .unwrap_or(false)
}

pub fn provide_theme_context(cx: Scope) {
  if use_context::<ThemeContext>(cx).is_none() {
    let initial = initial_theme(cx);

    let set_dark_action = create_server_action::<SetDarkTheme>(cx);

    let input = set_dark_action.input();
    let value = set_dark_action.value();

    let is_dark = Signal::derive(cx, move || {
      match (input(), value()) {
        // use current input optimistically
        (Some(submission), _) => submission.is_dark,
        // or previous value confirmed by server
        (_, Some(Ok(value))) => value,
        // or initial value
        _ => initial,
      }
    });

    provide_context(
      cx,
      ThemeContext {
        set_dark_action,
        is_dark,
      },
    );
  }
}

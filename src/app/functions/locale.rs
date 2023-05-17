use leptos::*;

use crate::app::i18n::Locale;

#[server(SetLocale, "/api")]
pub async fn set_locale(
  #[allow(unused_variables)] cx: Scope,
  locale: Locale,
) -> Result<Locale, ServerFnError> {
  use axum::http::header::{HeaderMap, HeaderValue, SET_COOKIE};
  use leptos_axum::{ResponseOptions, ResponseParts};

  let locale_str = locale.as_ref();
  let response = expect_context::<ResponseOptions>(cx);
  let mut response_parts = ResponseParts::default();
  let mut headers = HeaderMap::new();
  headers.insert(
    SET_COOKIE,
    HeaderValue::from_str(&format!("locale={locale_str}; Path=/")).expect("to create header value"),
  );
  response_parts.headers = headers;

  response.overwrite(response_parts);
  Ok(locale)
}

use leptos::*;

#[server(SetDarkTheme, "/api")]
pub async fn set_dark_theme(
  #[allow(unused_variables)] cx: Scope,
  is_dark: bool,
) -> Result<bool, ServerFnError> {
  use axum::http::header::{HeaderMap, HeaderValue, SET_COOKIE};
  use leptos_axum::{ResponseOptions, ResponseParts};

  let response =
    use_context::<ResponseOptions>(cx).expect("to have leptos_axum::ResponseOptions provided");
  let mut response_parts = ResponseParts::default();
  let mut headers = HeaderMap::new();
  headers.insert(
    SET_COOKIE,
    HeaderValue::from_str(&format!("darkmode={is_dark}; Path=/")).expect("to create header value"),
  );
  response_parts.headers = headers;

  response.overwrite(response_parts);
  Ok(is_dark)
}

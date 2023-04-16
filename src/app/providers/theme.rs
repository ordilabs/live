use std::*;

use crate::app::SetTheme;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl fmt::Display for Theme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Theme::Light => write!(f, "Light"),
            Theme::Dark => write!(f, "Dark"),
        }
    }
}

#[derive(Clone)]
pub(crate) struct ThemeContext {
    pub set_theme_action: Action<SetTheme, Result<Theme, ServerFnError>>,
    pub theme: Signal<Theme>,
}

#[cfg(not(feature = "ssr"))]
fn initial_theme(_cx: Scope) -> Theme {
    use wasm_bindgen::JsCast;

    let doc = document().unchecked_into::<web_sys::HtmlDocument>();
    let cookie = doc.cookie().unwrap_or_default();
    match &cookie {
        c if c.contains("theme=Light") => Theme::Light,
        c if c.contains("theme=Dark") => Theme::Dark,
        _ => Theme::Light,
    }
}

#[cfg(feature = "ssr")]
fn initial_theme(cx: Scope) -> Theme {
    use axum_extra::extract::cookie::CookieJar;

    use_context::<leptos_axum::RequestParts>(cx)
        .and_then(|req| {
            let cookies = CookieJar::from_headers(&req.headers);
            return cookies.get("theme").and_then(|v| match v.value() {
                "Light" => Some(Theme::Light),
                "Dark" => Some(Theme::Dark),
                _ => Some(Theme::Light),
            });
        })
        .unwrap_or(Theme::Light)
}

pub fn provide_theme_context(cx: Scope) {
    if use_context::<ThemeContext>(cx).is_none() {
        let initial = initial_theme(cx);

        let set_theme_action = create_server_action::<SetTheme>(cx);

        let input = set_theme_action.input();
        let value = set_theme_action.value();

        let theme = Signal::derive(cx, move || {
            match (input(), value()) {
                // use current input optimistically
                (Some(submission), _) => submission.theme,
                // or previous value confirmed by server
                (_, Some(Ok(value))) => value,
                // or initial value
                _ => initial,
            }
        });

        provide_context(
            cx,
            ThemeContext {
                set_theme_action,
                theme,
            },
        );
    }
}

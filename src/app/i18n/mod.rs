mod context;
mod de;
mod en;
mod hi;
mod zh_hans;
mod zh_hant;

mod types;

pub use context::*;
use leptos::{expect_context, Memo, Scope};
pub use types::*;

#[allow(dead_code)]
pub fn translation(l: Locale) -> Translation {
  match l {
    Locale::De => de::translation(),
    Locale::En => en::translation(),
    Locale::ZnHans => zh_hans::translation(),
    Locale::ZnHant => zh_hant::translation(),
    Locale::Hi => hi::translation(),
  }
}

#[allow(dead_code)]
pub fn t_macro(cx: Scope, t: T) -> Memo<String> {
  let i18n: I18nContext = expect_context::<I18nContext>(cx);
  i18n.translate(cx, t, None)
}

#[allow(dead_code)]
pub fn t_macro_with_args(cx: Scope, t: T, args: TranslationArgs) -> Memo<String> {
  let i18n: I18nContext = expect_context::<I18nContext>(cx);
  i18n.translate(cx, t, Some(args))
}

/// `t!` macro
///
/// # Arguments
///
/// * `cx` - Scope
/// * `t` - T (key of `Locale`)
/// * `args` - `{key = value}` (optional) - Optional `args` will fill placeholders in translations
///
/// # Examples
///
/// ```
/// use crate::t;
///
/// <div>{t!(cx, T::HomeTitle)}</div>
///
/// <h2>{move || {
///    
///    let a = "Alice".to_string();
///    let b = "Bob".to_string();
///    t!(cx, T::Hello, { "name_a" = a, "name_b" = "b" })
///    // Given T::Hello = "Hello {a} and {b}" output will be `Hello Alice and Bob`
/// </h2>
/// ```
#[macro_export]
macro_rules! t {
    // NO arguments
    ($cx:expr, $t:path) => {
        $crate::app::i18n::t_macro($cx, $t)
    };
    // WITH ARGs
    ($cx:expr, $t:path, {
        $($key:literal = $value:expr),+
    }) => {{
        let mut args: TranslationArgs = std::collections::HashMap::new();
        $(
            args.insert($key, $value);
        )+
        $crate::app::i18n::t_macro_with_args($cx, $t, args)
    }};
}

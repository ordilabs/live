use leptos::*;

use crate::app::functions::SetLocale;

use super::{
  translation,
  types::{Locale, T},
};

pub type SetLocaleAction = Action<SetLocale, Result<Locale, ServerFnError>>;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub(crate) struct I18nContext {
  pub locale: Signal<Locale>,
  pub set_locale_action: SetLocaleAction,
}

#[allow(dead_code)]
impl I18nContext {
  pub fn new(locale: Signal<Locale>, set_locale_action: SetLocaleAction) -> Self {
    Self {
      locale,
      set_locale_action,
    }
  }

  pub fn t(self, cx: Scope, key: T) -> Memo<String> {
    create_memo(cx, move |_| {
      let l = self.locale.get();
      if let Some(val) = translation(l).get(&key) {
        val.to_string()
      } else {
        debug_warn!("(i18n::t) key not found: {:?}", &key);
        format!("{:?}", key)
      }
    })
  }
}

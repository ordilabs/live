use leptos::*;

use crate::app::i18n::{translation, Locale, T};

#[allow(dead_code)]
#[derive(Clone)]
pub(crate) struct I18nContext {
  pub locale: RwSignal<Locale>,
}

impl I18nContext {
  pub fn new(cx: Scope, l: Locale) -> Self {
    let locale = create_rw_signal(cx, l);
    Self { locale }
  }

  pub fn t(self, cx: Scope, key: T) -> Memo<String> {
    create_memo(cx, move |_| {
      let l = self.locale.get();
      if let Some(val) = translation(l).get(&key) {
        val.to_string()
      } else {
        debug_warn!("(i18n::t) key not found: {:?}", key);
        format!("{:?}", key)
      }
    })
  }
}

pub fn provide_i18n_context(cx: Scope) {
  if use_context::<I18nContext>(cx).is_none() {
    provide_context(cx, I18nContext::new(cx, Locale::default()));
  }
}

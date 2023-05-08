use leptos::*;
use std::collections::HashMap;
use strum::{EnumIter, EnumString};

use crate::app::i18n::locale_data;

#[allow(dead_code)]
#[derive(Clone, EnumIter, EnumString, Debug, PartialEq, Eq)]
pub enum Locale {
  EN,
  DE,
}

impl Locale {
  pub fn as_str(&self) -> &'static str {
    match self {
      Locale::EN => "EN",
      Locale::DE => "DE",
    }
  }
}

pub type Translation = HashMap<TK, &'static str>;

// TK = Translation Key
#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum TK {
  Hello,
  World,
}

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

  pub fn t(self, cx: Scope, key: TK) -> Memo<String> {
    create_memo(cx, move |_| {
      let l = self.locale.get();
      if let Some(val) = locale_data(l).get(&key) {
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
    provide_context(cx, I18nContext::new(cx, Locale::EN));
  }
}

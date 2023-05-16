use leptos::*;

use crate::app::functions::SetLocale;

use std::option::Option;

use super::{
  translation,
  types::{Locale, T},
  TranslationArgs,
};

pub type SetLocaleAction = Action<SetLocale, Result<Locale, ServerFnError>>;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub(crate) struct I18nContext {
  pub locale: Signal<Locale>,
  pub set_locale_action: SetLocaleAction,
}

pub fn replace(trans: String, args: &TranslationArgs) -> String {
  let mut trans = trans;
  for (&k, v) in args.iter() {
    // Replace `${<k>}` -> `v`
    trans = trans.replace(&format!("{{${}}}", k), v);
  }
  trans.to_owned()
}

#[allow(dead_code)]
impl I18nContext {
  pub fn new(locale: Signal<Locale>, set_locale_action: SetLocaleAction) -> Self {
    Self {
      locale,
      set_locale_action,
    }
  }

  pub fn translate(self, cx: Scope, t: T, args: Option<TranslationArgs>) -> Memo<String> {
    create_memo(cx, move |_| match translation(self.locale.get()).get(&t) {
      Some(&trans) => {
        let mut trans = trans.to_string();
        if let Some(args) = &args {
          trans = replace(trans.clone(), args);
        }
        trans.to_string()
      }
      None => {
        debug_warn!("(i18n::t) key not found: {:?}", &t);
        format!("{:?}", t)
      }
    })
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use crate::app::i18n::replace;
  use common_macros::hash_map;

  #[test]
  fn replace_one_arg() {
    let t = "hello {$name}".to_string();
    let h = hash_map! {"name" => "world".to_string()};
    let result = replace(t, &h);
    assert_eq!(result, "hello world");
  }

  #[test]
  fn replace_two_args() {
    let t = "hello {$1} and {$2}".to_string();
    let h = hash_map! {"1" => "Bob".to_string(), "2" => "Alice".to_string()};
    let result = replace(t, &h);
    assert_eq!(result, "hello Bob and Alice");
  }

  #[test]
  fn replace_wrong_param() {
    let t = "hello {$name}".to_string();
    let h = hash_map! {"unkown" => "bbbr".to_string()};
    let result = replace(t, &h);
    assert_eq!(result, "hello {$name}");
  }

  #[test]
  fn replace_empty_map() {
    let t = "hello {$name}".to_string();
    let result = replace(t, &HashMap::new());
    assert_eq!(result, "hello {$name}");
  }

  #[test]
  fn replace_nothing() {
    let t = "hello".to_string();
    let h = hash_map! {"name" => "world".to_string()};
    let result = replace(t, &h);
    assert_eq!(result, "hello");
  }
}

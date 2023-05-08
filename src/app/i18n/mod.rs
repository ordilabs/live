use crate::app::providers::i18n::Locale;

use super::providers::Translation;

mod de;
mod en;

#[allow(dead_code)]
pub fn locale_data(l: Locale) -> Translation {
  match l {
    Locale::DE => de::translation(),
    Locale::EN => en::translation(),
  }
}

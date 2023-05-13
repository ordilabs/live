mod de;
mod en;
mod hi;
mod zh_hans;
mod zh_hant;

mod types;

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

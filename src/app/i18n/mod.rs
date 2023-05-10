mod de;
mod en;
mod hi;
mod zn;

mod types;

pub use types::*;

#[allow(dead_code)]
pub fn translation(l: Locale) -> Translation {
  match l {
    Locale::DE => de::translation(),
    Locale::EN => en::translation(),
    Locale::ZN => zn::translation(),
    Locale::HI => hi::translation(),
  }
}

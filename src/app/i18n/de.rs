use crate::app::providers::i18n::{Translation, TK};

use common_macros::hash_map;

#[allow(dead_code)]
pub fn translation() -> Translation {
  hash_map!(
      TK::Hello => "Hallo",
      TK::World => "Welt",
  )
}

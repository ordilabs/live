use common_macros::hash_map;

use super::types::{Translation, T};

#[allow(dead_code)]
pub fn translation() -> Translation {
  hash_map!(
      T::HomeTitle => "Erwartete Inschriften",
      T::ForkGH => "Fork bei GitHub",
  )
}

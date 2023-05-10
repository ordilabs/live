use common_macros::hash_map;

use super::types::{Translation, T};

#[allow(dead_code)]
pub fn translation() -> Translation {
  hash_map!(
    T::HomeTitle => "Anticipated inscriptions",
    T::ForkGH => "Fork on GitHub",
  )
}

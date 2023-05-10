use common_macros::hash_map;

use super::types::{Translation, T};

pub fn translation() -> Translation {
  hash_map!(
    T::HomeTitle => "預期的銘文",
    T::ForkGH => "在GitHub上派生",
  )
}

use common_macros::hash_map;

use super::types::{Translation, T};

pub fn translation() -> Translation {
  hash_map!(
    T::HomeTitle => "अपेक्षित अभिलेख",
    T::ForkGH => "गिटहब पर फोर्क",
  )
}

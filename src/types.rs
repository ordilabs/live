use std::cmp::Ordering;

use ord_labs::Media;
use serde::{Deserialize, Serialize};

// Deriving De/Serialize for `Media` (different crate)
// https://serde.rs/remote-derive.html#deriving-deserialize-for-type-in-a-different-crate
#[derive(Serialize, Deserialize)]
#[serde(remote = "Media")]
enum MediaDef {
  Audio,
  Iframe,
  Image,
  Pdf,
  Text,
  Unknown,
  Video,
}

// Custom ordering - it overrides original `Ord` of Media
pub fn compare_media(a: &Media, b: &Media) -> Ordering {
  media_value(a).cmp(&media_value(b))
}

// Custom value of Media - it is used for custom ordering
pub fn media_value(&m: &Media) -> usize {
  match m {
    Media::Image => 0,
    Media::Text => 1,
    Media::Audio => 2,
    Media::Video => 3,
    Media::Pdf => 4,
    Media::Iframe => 5,
    Media::Unknown => 6,
  }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize, Copy)]
pub struct MempoolInfo {
  #[serde(with = "MediaDef")]
  pub media: Media,
  pub count: usize,
  pub size: usize,
}

pub type MempoolAllInfo = Vec<MempoolInfo>;

#[cfg(feature = "ssr")]
#[derive(Clone, Debug)]
pub enum LiveEvent {
  NewInscription(String),
  RandomInscription(String),
  MempoolInfo(MempoolAllInfo),
  BlockCount(u64),
}

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
  // TODO (@sectore) Remove it - just for testing serialization/deserialization LiveEvents (see #100)
  ServerTime(std::time::SystemTime),
}

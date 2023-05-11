use common_macros::hash_map;

use super::types::{Translation, T};

#[allow(dead_code)]
pub fn translation() -> Translation {
  hash_map!(
    T::HomeTitle => "Anticipated inscriptions",
    T::ForkGH => "Fork on GitHub",
    T::Audio => "audio",
    T::Audios => "audios",
    T::Video => "video",
    T::Videos => "videos",
    T::Image => "image",
    T::Images => "images",
    T::Pdf => "PDF",
    T::Pdfs => "PDF's",
    T::Unknown => "Unknown",
    T::Unknowns => "Unknowns",
    T::Text => "Text",
    T::Texts => "Texts",
    T::Iframe => "IFrame",
    T::Iframes => "IFrames",
  )
}

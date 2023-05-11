use common_macros::hash_map;

use super::types::{Translation, T};

pub fn translation() -> Translation {
  hash_map!(
    T::HomeTitle => "預期的銘文",
    T::ForkGH => "在GitHub上派生",
    T::Audio => "audio - zn",
    T::Audios => "audios - zn",
    T::Video => "video - zn",
    T::Videos => "videos - zn",
    T::Image => "image - zn",
    T::Images => "images - zn",
    T::Pdf => "PDF - zn",
    T::Pdfs => "PDF's - zn",
    T::Unknown => "Unknown - zn",
    T::Unknowns => "Unknowns - zn",
    T::Text => "Text - zn",
    T::Texts => "Texts - zn",
    T::Iframe => "IFrame - zn",
    T::Iframes => "IFrames - zn",
  )
}

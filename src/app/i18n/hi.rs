use common_macros::hash_map;

use super::types::{Translation, T};

pub fn translation() -> Translation {
  hash_map!(
    T::HomeTitle => "अपेक्षित अभिलेख",
    T::ForkGH => "गिटहब पर फोर्क",
    T::Audio => "audio - hi",
    T::Audios => "audios - hi",
    T::Video => "video - hi",
    T::Videos => "videos - hi",
    T::Image => "image - hi",
    T::Images => "images - hi",
    T::Pdf => "PDF - hi",
    T::Pdfs => "PDF's - hi",
    T::Unknown => "Unknown - hi",
    T::Unknowns => "Unknowns - hi",
    T::Text => "Text - hi",
    T::Texts => "Texts - hi",
    T::Iframe => "IFrame - hi",
    T::Iframes => "IFrames - hi",
  )
}

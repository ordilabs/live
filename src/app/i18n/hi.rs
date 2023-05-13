use common_macros::hash_map;

use super::types::{Translation, T};

// हिंदी (Hindi)
pub fn translation() -> Translation {
  hash_map!(
    T::HomeTitle => "अपेक्षित अभिलेख",
    T::ForkGH => "गिटहब पर फोर्क",
    T::Audio => "ऑडियो",
    T::Audios => "ऑडियो",
    T::Video => "वीडियो",
    T::Videos => "वीडियो",
    T::Image => "छवि",
    T::Images => "छवियाँ",
    T::Pdf => "PDF",
    T::Pdfs => "PDFs",
    T::Unknown => "अज्ञात",
    T::Unknowns => "अज्ञात",
    T::Text => "पाठ",
    T::Texts => "पाठ",
    T::Iframe => "आईफ्रेम",
    T::Iframes => "आईफ्रेम",
  )
}

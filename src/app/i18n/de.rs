use common_macros::hash_map;

use super::types::{Translation, T};

// Deutsch (German)
pub fn translation() -> Translation {
  hash_map!(
    T::HomeTitle => "Erwartete Inschriften",
    T::ForkGH => "Fork bei GitHub",
    T::Audio => "Audio",
    T::Audios => "Audios",
    T::Video => "Video",
    T::Videos => "Videos",
    T::Image => "Bild",
    T::Images => "Bilder",
    T::Pdf => "PDF",
    T::Pdfs => "PDF's",
    T::Unknown => "Unbekannt",
    T::Unknowns => "Unbekannte",
    T::Text => "Text",
    T::Texts => "Texte",
    T::Iframe => "IFrame",
    T::Iframes => "IFrames",
  )
}

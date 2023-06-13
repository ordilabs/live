use common_macros::hash_map;

use super::types::{Translation, T};

// English
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
    T::Pdf => "pdf",
    T::Pdfs => "pdf's",
    T::Unknown => "unknown",
    T::Unknowns => "unknowns",
    T::Text => "text",
    T::Texts => "texts",
    T::Iframe => "iframe",
    T::Iframes => "iframes",
    T::Id => "id",
    T::Address => "address",
    T::OutputValue => "output value",
    T::Sat => "sat",
    T::ContentLength => "content length",
    T::ContentType => "content type",
    T::TimeStamp => "time stamp",
    T::GenesisHeight => "genesis height",
    T::GenesisFee => "genesis fee",
    T::GenesisTx => "genesis transaction",
    T::Location => "location",
    T::Output => "output",
    T::Offset => "offset",
  )
}

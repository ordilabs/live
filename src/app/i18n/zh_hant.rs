use common_macros::hash_map;

use super::types::{Translation, T};

// Chinese (Traditional)
pub fn translation() -> Translation {
  hash_map!(
    T::HomeTitle => "預期的銘文",
    T::ForkGH => "在GitHub上派生",
    T::Audio => "音頻",
    T::Audios => "音頻集",
    T::Video => "視頻",
    T::Videos => "視頻集",
    T::Image => "圖片",
    T::Images => "圖片集",
    T::Pdf => "PDF",
    T::Pdfs => "PDF集",
    T::Unknown => "未知",
    T::Unknowns => "未知集",
    T::Text => "文字",
    T::Texts => "文字集",
    T::Iframe => "框架",
    T::Iframes => "框架集",
  )
}

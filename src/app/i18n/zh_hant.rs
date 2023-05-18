use common_macros::hash_map;

use super::types::{Translation, T};

// 繁體中文 (Traditional Chinese)
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
    T::Id => "id",
    T::Address => "地址",
    T::OutputValue => "輸出值",
    T::Sat => "聰",
    T::ContentLength => "內容長度",
    T::ContentType => "內容類型",
    T::TimeStamp => "時間戳",
    T::GenesisHeight => "創世區塊高度",
    T::GenesisFee => "創世費用",
    T::GenesisTx => "創世交易",
    T::Location => "位置",
    T::Output => "輸出",
    T::Offset => "抵消",
  )
}

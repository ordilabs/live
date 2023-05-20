use common_macros::hash_map;

use super::types::{Translation, T};

// 简体中文 (Simplified Chinese)
pub fn translation() -> Translation {
  hash_map!(
    T::HomeTitle => "预期的铭文",
    T::ForkGH => "在GitHub上派生",
    T::Audio => "音频",
    T::Audios => "音频集",
    T::Video => "视频",
    T::Videos => "视频集",
    T::Image => "图片",
    T::Images => "图片集",
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
    T::OutputValue => "输出值",
    T::Sat => "聪",
    T::ContentLength => "内容长度",
    T::ContentType => "内容类型",
    T::TimeStamp => "时间戳",
    T::GenesisHeight => "创世区块高度",
    T::GenesisFee => "创世费用",
    T::GenesisTx => "创世交易",
    T::Location => "位置",
    T::Output => "输出",
    T::Offset => "抵消",
  )
}

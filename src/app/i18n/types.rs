use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::{AsRefStr, EnumIter, EnumString};

#[allow(dead_code)]
#[derive(
  Clone, EnumIter, AsRefStr, EnumString, Debug, PartialEq, Eq, Default, Serialize, Deserialize, Copy,
)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum Locale {
  #[default]
  En,
  De,
  // Chinese (Simplified)
  ZnHans,
  // Chinese (Traditional)
  ZnHant,
  Hi,
}

#[allow(dead_code)]
impl Locale {
  pub fn to_label(&self) -> &'static str {
    match self {
      Locale::En => "EN",
      Locale::De => "DE",
      Locale::ZnHans => "简体",
      Locale::ZnHant => "繁體",
      Locale::Hi => "हिंदी",
    }
  }
}

// T = Translation Key
#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum T {
  HomeTitle,
  ForkGH,
  Audio,
  Audios,
  Video,
  Videos,
  Pdf,
  Pdfs,
  Unknown,
  Unknowns,
  Image,
  Images,
  Text,
  Texts,
  Iframe,
  Iframes,
  Id,
  Address,
  OutputValue,
  Sat,
  ContentLength,
  ContentType,
  TimeStamp,
  GenesisHeight,
  GenesisFee,
  GenesisTx,
  Location,
  Output,
  Offset,
}

pub type Translation = HashMap<T, &'static str>;

pub type TranslationArgs = HashMap<&'static str, String>;

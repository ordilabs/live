use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::{EnumIter, EnumString};

#[allow(dead_code)]
#[derive(
  Clone, EnumIter, EnumString, Debug, PartialEq, Eq, Default, Serialize, Deserialize, Copy,
)]
pub enum Locale {
  #[default]
  EN,
  DE,
  ZN,
  HI,
}

#[allow(dead_code)]
impl Locale {
  pub fn as_str(&self) -> &'static str {
    match self {
      Locale::EN => "EN",
      Locale::DE => "DE",
      Locale::ZN => "ZN",
      Locale::HI => "HI",
    }
  }
}

// T = Translation Key
#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub enum T {
  HomeTitle,
  ForkGH,
}

pub type Translation = HashMap<T, &'static str>;

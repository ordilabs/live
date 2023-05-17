extern crate async_trait;
extern crate ord_labs;
extern crate serde;

pub use async_trait::async_trait;
pub use bitcoin::Transaction;
pub use ord_labs::{Inscription, Media};
use serde::Deserialize;
pub mod bitcoin_core;
pub use bitcoin_core::BitcoinCore;

use anyhow::Result;

#[derive(Deserialize, Debug)]
pub struct MempoolRecentEntry {
  pub txid: String,
  pub fee: u64,
  pub vsize: f64,
  pub value: u64,
}

pub type MempoolRecent = Vec<MempoolRecentEntry>;

#[async_trait]
pub trait Backend {
  fn new() -> Self;
  async fn get_latest_inscriptions(&self) -> Vec<Inscription>;

  async fn recent(&self) -> Result<MempoolRecent>;

  async fn maybe_inscription(&self, txid: &str) -> Result<Option<Inscription>>;
}

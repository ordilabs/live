//use tokio::task;
extern crate bitcoin;
extern crate bitcoincore_rpc;
extern crate bitcoincore_rpc_json;
extern crate directories;
use directories::BaseDirs;

use std::{env::var, path::PathBuf, str::FromStr};

use super::*;
use anyhow::Result;

use bitcoincore_rpc::*;
use bitcoincore_rpc_json::*;

//use bitcoin::bitcoincore_rpc_json::GetBlockTemplateResult;
#[derive(Debug, Clone)]
pub struct BitcoinCore {
  auth: bitcoincore_rpc::Auth,
  root: String,
  //client: bitcoincore_rpc::Client,
}

//#[async_trait]
impl BitcoinCore {
  pub fn new() -> Self {
    let core_url = var("CORE_URL").unwrap_or("127.0.0.1:8332".to_owned());

    let auth = match var("CORE_USER").ok() {
      None => {
        let mut fallback = bitcoin_dir(bitcoin::Network::Bitcoin);
        fallback.push(".cookie");
        let core_cookie = var("CORE_COOKIE").unwrap_or(fallback.to_str().unwrap().to_owned());
        let home = var("HOME").expect("env HOME to be set");
        let path = core_cookie.replace("~", &home);
        let path = PathBuf::from_str(&path).expect("a valid path");
        bitcoincore_rpc::Auth::CookieFile(path)
      }
      Some(core_user) => {
        let core_pass = var("CORE_PASS").unwrap();
        bitcoincore_rpc::Auth::UserPass(core_user, core_pass)
      }
    };

    BitcoinCore {
      root: core_url,
      auth,
    }
  }

  pub async fn get_block_count(&self) -> u64 {
    let client = bitcoincore_rpc::Client::new(&self.root, self.auth.clone()).unwrap();
    client.get_block_count().unwrap()
  }

  pub async fn _get_latest_inscriptions(&self) -> Vec<Inscription> {
    let mut inscriptions = Vec::new();
    let mpr = self.recent().await.ok();

    match mpr {
      Some(mpr) => {
        for entry in mpr {
          let txid = entry.txid;
          let maybe_inscription = self.maybe_inscription(&txid).await.unwrap();
          if maybe_inscription.is_some() {
            inscriptions.push(maybe_inscription.unwrap());
          }
        }
      }
      _ => {}
    }

    inscriptions
  }

  pub async fn recent(&self) -> Result<MempoolRecent> {
    let client = bitcoincore_rpc::Client::new(&self.root, self.auth.clone()).unwrap();
    let grm = client.get_raw_mempool()?;

    let mpr = grm
      .iter()
      .map(|txid| {
        let txid = txid.to_string();
        MempoolRecentEntry {
          txid,
          fee: 0,
          vsize: 0.,
          value: 0,
        }
      })
      .collect();

    Ok(mpr)
  }

  pub async fn maybe_inscription(&self, txid: &str) -> Result<Option<Inscription>> {
    let client = bitcoincore_rpc::Client::new(&self.root, self.auth.clone()).unwrap();
    //let txid = bitcoin::Txid::from_hex(txid).unwrap();
    let txid = txid.parse::<bitcoincore_rpc::bitcoin::Txid>().unwrap();

    // let hex = client.get_raw_transaction(&txid, None)?;
    // let data = hex::decode(&hex)?;
    // let transaction: Transaction = bitcoin::consensus::deserialize(&data)?;
    let transaction = client.get_raw_transaction(&txid, None)?;
    //dbg!(&transaction);
    let maybe_inscription = Inscription::from_transaction(&transaction);
    Ok(maybe_inscription)
  }

  pub async fn maybe_inscription_with_index(
    &self,
    txid: &str,
    index: usize,
  ) -> Result<Option<Inscription>> {
    let client = bitcoincore_rpc::Client::new(&self.root, self.auth.clone()).unwrap();
    //let txid = bitcoin::Txid::from_hex(txid).unwrap();
    let txid = txid.parse::<bitcoincore_rpc::bitcoin::Txid>().unwrap();

    // let hex = client.get_raw_transaction(&txid, None)?;
    // let data = hex::decode(&hex)?;
    // let transaction: Transaction = bitcoin::consensus::deserialize(&data)?;
    let transaction = client.get_raw_transaction(&txid, None)?;
    //dbg!(&transaction);
    let v = Inscription::from_transaction_vec(&transaction);
    if v.len() < index {
      return Ok(None); // out of bounds
    }
    Ok(v[index].clone())
  }
}

impl BitcoinCore {
  async fn _recent_gbt(&self) -> Result<MempoolRecent> {
    let mode = GetBlockTemplateModes::Template;
    let rules = [GetBlockTemplateRules::SegWit];
    let capabilities = [];

    let client = bitcoincore_rpc::Client::new(&self.root, self.auth.clone()).unwrap();
    let gtr = client.get_block_template(mode, &rules, &capabilities)?;

    let mpr = gtr
      .transactions
      .iter()
      .map(|tx| {
        let txid = tx.txid;
        let fee = tx.fee.to_sat();
        let vsize: f64 = tx.weight as f64 / 4.;
        let value = 0;

        let txid = txid.to_string();
        MempoolRecentEntry {
          txid,
          fee,
          vsize,
          value,
        }
      })
      .collect();
    Ok(mpr)
  }
}

pub fn bitcoin_dir(network: bitcoin::Network) -> PathBuf {
  let mut bitcoin_dir: PathBuf = BaseDirs::new().unwrap().data_dir().into();

  match std::env::consts::OS {
    "windows" | "macos" => {
      bitcoin_dir.push("Bitcoin");
    }
    _ => {
      bitcoin_dir.push(".bitcoin");
    }
  };

  match network {
    bitcoin::Network::Bitcoin => {}
    bitcoin::Network::Testnet => {
      bitcoin_dir.push("testnet3");
    }
    bitcoin::Network::Regtest => {
      bitcoin_dir.push("regtest");
    }
    bitcoin::Network::Signet => {
      bitcoin_dir.push("signet");
    }
    _ => todo!(),
  };

  return bitcoin_dir;
}

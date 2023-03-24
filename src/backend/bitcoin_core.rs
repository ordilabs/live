//use tokio::task;
extern crate bitcoin;
extern crate bitcoincore_rpc;
extern crate bitcoincore_rpc_json;

use std::{env::var, path::PathBuf, str::FromStr};

use super::*;
use anyhow::Result;

use bitcoin::hashes::hex::FromHex;
use bitcoincore_rpc::*;
use bitcoincore_rpc_json::*;

//use bitcoin::bitcoincore_rpc_json::GetBlockTemplateResult;
#[derive(Debug, Clone)]
pub struct BitcoinCore {
    auth: bitcoincore_rpc::Auth,
    root: String,
    //client: bitcoincore_rpc::Client,
}

#[async_trait]
impl Backend for BitcoinCore {
    fn new() -> Self {
        let core_url = var("CORE_URL").unwrap_or("127.0.0.1:18443".to_owned());

        let auth = match var("CORE_USER").ok() {
            None => {
                let fallback = "docker/bitcoin.cookie".to_owned();
                let core_cookie = var("CORE_COOKIE").unwrap_or(fallback);
                let home = var("HOME").expect("HOME to be set");
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

    async fn get_latest_inscriptions(&self) -> Vec<Inscription> {
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

    async fn recent(&self) -> Result<MempoolRecent> {
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

    async fn maybe_inscription(&self, txid: &str) -> Result<Option<Inscription>> {
        let client = bitcoincore_rpc::Client::new(&self.root, self.auth.clone()).unwrap();
        let txid = bitcoin::Txid::from_hex(txid).unwrap();

        // let hex = client.get_raw_transaction(&txid, None)?;
        // let data = hex::decode(&hex)?;
        // let transaction: Transaction = bitcoin::consensus::deserialize(&data)?;
        let transaction = client.get_raw_transaction(&txid, None)?;
        //dbg!(&transaction);
        let maybe_inscription = Inscription::from_transaction(&transaction);
        Ok(maybe_inscription)
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

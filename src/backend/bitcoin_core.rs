//use tokio::task;
extern crate bitcoin;
extern crate bitcoincore_rpc;
extern crate bitcoincore_rpc_json;

use std::{mem::replace, path::PathBuf};

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
        let root = std::env::var("CORE_URL").unwrap_or("127.0.0.1:18443".to_owned());

        let core_user = std::env::var("CORE_USER").unwrap_or("mempool".to_owned());
        let core_pass = std::env::var("CORE_PASS").unwrap_or("mempool".to_owned());

        let core_cookie = std::env::var("CORE_COOKIE").unwrap_or("~/.bitcoin/.cookie".to_owned());
        let home = std::env::var("HOME").expect("HOME to be set");
        let core_cookie = core_cookie.replace("~", &home);

        let auth = match core_cookie != "" {
            true => {
                let path = PathBuf::from(core_cookie);
                bitcoincore_rpc::Auth::CookieFile(path)
            }
            false => bitcoincore_rpc::Auth::UserPass(core_user.clone(), core_pass.clone()),
        };
        //let cookie = std::env::var("CORE_COOKIE").unwrap_or("".to_owned());

        //  = bitcoincore_rpc::Auth::UserPass(core_user.clone(), core_pass.clone());

        BitcoinCore { root, auth }
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

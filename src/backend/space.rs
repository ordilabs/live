use super::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Space {
    // The actual memory pool
    root: String,
    // The current position in the pool
}
const SPACE_ROOT: &str = "http://mempool-ol.local";

#[async_trait]
impl Backend for Space {
    // impl Space {
    fn new() -> Self {
        let root = std::env::var("SPACE_ROOT").unwrap_or(SPACE_ROOT.to_owned());
        Space { root }
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
        let url = self.root.clone() + "/api/mempool/recent";
        let mpr = reqwest::get(url).await?.json::<MempoolRecent>().await?;
        Ok(mpr)
    }

    async fn maybe_inscription(&self, txid: &str) -> Result<Option<Inscription>> {
        let url = self.root.to_owned() + &"/api/tx/{}/hex".replace("{}", &txid);

        let hex = reqwest::get(url).await.unwrap().text().await.unwrap();
        let data = hex::decode(&hex)?;
        let transaction: Transaction = bitcoin::consensus::deserialize(&data)?;
        let maybe_inscription = Inscription::from_transaction(&transaction);
        Ok(maybe_inscription)
    }
}

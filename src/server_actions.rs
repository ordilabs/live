use std::collections::HashMap;

use super::*;
use crate::ord::inscription::Inscription;
use crate::ord::media::Media;

use actix_web::http::header::{self, HeaderValue};
use bitcoin::*;
use serde::*;

//use std::io::Read::read_to_end;

#[derive(Deserialize)]
pub struct Content {
    pub inscription_id: String,
}

#[derive(Deserialize, Debug)]
pub struct MempoolRecentEntry {
    pub txid: String,
    pub fee: u64,
    pub vsize: f64,
    pub value: u64,
}

const MEMPOOL_API_URL: &str = "http://mempool-ol.local/";

//#[derive(Deserialize)]
pub type MempoolRecent = Vec<MempoolRecentEntry>;

pub(crate) async fn tick(ordipool: &mut HashMap<String, Option<Inscription>>) {
    let mpr = recent().await.ok();

    match mpr {
        Some(mpr) => {
            for entry in mpr {
                let txid = entry.txid;
                if ordipool.contains_key(&txid) {
                    continue;
                }
                let maybe_inscription = mempool_maybe_inscription(&txid).await.unwrap();
                if maybe_inscription.is_some() {
                    let ins = format!("{}i0", &txid);
                    _ = INSCRIPTION_CHANNEL.send(&ins).await;
                    log!("broadcast {}", &txid);
                }
                _ = ordipool.entry(txid.clone()).or_insert(maybe_inscription);

                //dbg!("broadcasting {}", &txid);
            }
        }
        _ => {}
    }

    log!("tick");
}

pub async fn recent() -> Result<MempoolRecent, anyhow::Error> {
    let url = MEMPOOL_API_URL.to_owned() + "/api/mempool/recent";
    let mpr = reqwest::get(url).await?.json::<MempoolRecent>().await?;
    Ok(mpr)
}

pub async fn content(path: web::Path<Content>) -> impl Responder {
    let s = path.inscription_id.to_owned();
    if s.starts_with("punk") {
        let location = format!("/punks/{}", s);

        return HttpResponse::TemporaryRedirect()
            .insert_header((header::LOCATION, location))
            .content_type("text/plain")
            .body("body");
    }

    if s.len() < 64 {
        return HttpResponse::NotFound()
            .content_type("text/plain")
            .body("body");
    }
    //let path_ = path.inscription_id.clone();
    //let s = path.inscription_id.as_str();
    dbg!(&s);
    //log!("{}", s);

    let txid = &s.as_str()[0..64];

    // get content from remote server
    // todo gfi: use the /raw api instead of /hex

    let maybe_inscription = mempool_maybe_inscription(txid).await.unwrap();
    dbg!(&maybe_inscription);

    match maybe_inscription {
        Some(inscription) => match inscription.media() {
            Media::Image => image_response_actix(inscription),
            _ => HttpResponse::Ok()
                .content_type("text/plain")
                .body("Not an image"),
        },
        None => HttpResponse::NotFound()
            .content_type("text/plain")
            .body("Not found"),
    }
}

pub(crate) async fn mempool_maybe_inscription(
    txid: &str,
) -> Result<Option<Inscription>, anyhow::Error> {
    let url = (MEMPOOL_API_URL.to_owned() + "api/tx/{}/hex").replace("{}", txid);
    let hex = reqwest::get(url).await.unwrap().text().await.unwrap();
    let data = hex::decode(&hex)?;
    let transaction: Transaction = consensus::deserialize(&data)?;
    let maybe_inscription = Inscription::from_transaction(&transaction);
    Ok(maybe_inscription)
}

fn image_response_actix(inscription: Inscription) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(inscription.content_type().unwrap())
        .insert_header((
            header::CONTENT_SECURITY_POLICY,
            HeaderValue::from_static("default-src 'unsafe-eval' 'unsafe-inline' data:"),
        ))
        .insert_header((
            header::CACHE_CONTROL,
            HeaderValue::from_static("max-age=31536000, immutable"),
        ))
        .insert_header((
            header::CONTENT_LENGTH,
            inscription.content_length().unwrap_or_default(),
        ))
        .body(inscription.into_body().unwrap())
}

pub async fn preview(path: web::Path<Content>) -> impl Responder {
    let resp = r#"<!doctype html>
  <html lang=en>
    <head>
      <meta charset=utf-8>
      <meta name=format-detection content='telephone=no'>
      <style>
        html {
          background-color: #131516;
          height: 100%;
        }
  
        body {
          background-image: url(/content/{});
          background-position: center;
          background-repeat: no-repeat;
          background-size: contain;
          height: 100%;
          image-rendering: pixelated;
          margin: 0;
        }
  
        img {
          height: 100%;
          opacity: 0;
          width: 100%;
        }
      </style>
    </head>
    <body>
      <img src=/content/{}></img>
    </body>
  </html>
  "#;
    HttpResponse::Ok()
        .content_type("text/html")
        .body(resp.replace("{}", &path.inscription_id))
}

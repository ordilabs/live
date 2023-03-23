use super::*;
use std::collections::HashMap;
extern crate ord_mini;
extern crate rand;
use rand::seq::IteratorRandom;

use ord_mini::{Inscription, Media};

use actix_web::http::header::{self, HeaderValue};
use backend::{Backend, BitcoinCore, Space};
use serde::*;

//use std::io::Read::read_to_end;

#[derive(Deserialize)]
pub struct Content {
    pub inscription_id: String,
}

//#[derive(Deserialize)]

pub(crate) async fn tick_space(
    backend: &Space,
    ordipool: &mut HashMap<String, Option<Inscription>>,
) {
    let mpr = backend.recent().await.ok();

    let mut mpr_len = 0;
    match mpr {
        Some(mpr) => {
            mpr_len = mpr.len();
            for entry in mpr {
                let txid = entry.txid;
                if ordipool.contains_key(&txid) {
                    continue;
                }
                let maybe_inscription = backend.maybe_inscription(&txid).await.unwrap();
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

    log!("tick space, {}", &mpr_len);
}

pub(crate) async fn tick_bitcoin_core(
    backend: &BitcoinCore,
    ordipool: &mut HashMap<String, Option<Inscription>>,
) {
    let mpr = backend.recent().await.ok();

    let mut mpr_len = 0;
    let mut mpr_ins = 0;
    let mut mpr_img = 0;
    let mut broadcast = vec![];
    match mpr {
        Some(mpr) => {
            mpr_len = mpr.len();
            for (n, entry) in mpr.into_iter().enumerate() {
                let txid = entry.txid;
                if ordipool.contains_key(&txid) {
                    continue;
                }
                if n % 100 == 1 {
                    println!("processing {}/{}", n, mpr_len);
                }

                let maybe_inscription = backend.maybe_inscription(&txid).await.unwrap();

                if maybe_inscription.is_none() {
                    ordipool.entry(txid.clone()).or_insert(None);
                    continue;
                }

                mpr_ins += 1;
                let ins = maybe_inscription.unwrap();

                if ins.media() != Media::Image {
                    ordipool.entry(txid.clone()).or_insert(None);
                    continue;
                }

                mpr_img += 1;

                let inscription_id = format!("{}i0", &txid);
                //_ = INSCRIPTION_CHANNEL.send(&inscription_id).await;
                broadcast.push(inscription_id);
                //log!("broadcast {}", );

                ordipool.entry(txid.clone()).or_insert(Some(ins));
                //_ = ordipool.entry(txid.clone()).or_insert(maybe_inscription);

                //dbg!("broadcasting {}", &txid);
            }
        }
        _ => {}
    }

    if broadcast.len() > 4 {
        broadcast.resize(4, String::new());
    }

    if broadcast.len() == 0 {
        let chosen = ordipool
            .iter()
            .filter(|entry: &(&String, &Option<Inscription>)| entry.1.is_some())
            .choose(&mut rand::thread_rng());
        if chosen.is_some() {
            let txid = chosen.unwrap().0;

            log!("re-broadcast {}", &txid);
        }
    } else {
        for txid in broadcast {
            let ins = format!("{}i0", &txid);
            _ = INSCRIPTION_CHANNEL.send(&ins).await;
        }
    }

    log!(
        "tick: bitcoin_core, {}, {}, {}",
        &mpr_len,
        &mpr_ins,
        &mpr_img
    );
}

pub async fn content(path: web::Path<Content>) -> impl Responder {
    let s = path.inscription_id.to_owned();
    dbg!(&path.inscription_id);
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

    //dbg!(path.inscription_id.as_str());
    //let path_ = path.inscription_id.clone();
    //let s = path.inscription_id.as_str();
    //dbg!(&s);
    //log!("{}", s);

    let txid = &s.as_str()[0..64];
    let backend = BitcoinCore::new();
    dbg!("content for: {}", txid);

    // get content from remote server
    // todo gfi: use the /raw api instead of /hex

    let maybe_inscription = backend.maybe_inscription(txid).await.unwrap();

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

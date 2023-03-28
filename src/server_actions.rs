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
                    let event = LiveEvent::RandomInscription(format!("{}i0", &txid));
                    _ = EVENT_CHANNEL.send(&event).await;
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
    let mpr = mpr.unwrap_or_default();

    let mpr_len = mpr.len();
    let mut broadcast = vec![];

    // remove all non-mempool txs
    let txid: Vec<_> = mpr.iter().map(|entry| &entry.txid).collect();
    ordipool.retain(|key, _| txid.contains(&key));

    let mut backend_query_count = 0;

    for (_n, entry) in mpr.into_iter().enumerate() {
        let txid = entry.txid;
        if ordipool.contains_key(&txid) {
            continue;
        }

        let maybe_inscription = backend.maybe_inscription(&txid).await;
        backend_query_count += 1;

        let maybe_inscription = maybe_inscription.unwrap_or_else(|e| {
            log::warn!("{}", e);
            None
        });

        ordipool.insert(txid.clone(), maybe_inscription.clone());

        if maybe_inscription.is_none() {
            continue;
        }

        let inscription = maybe_inscription.unwrap();
        // only broadcast images for now
        if inscription.media() != Media::Image {
            continue;
        }

        let inscription_id = format!("{}i0", &txid);
        broadcast.push(inscription_id);

        // on startup, don't process too many per tick
        if backend_query_count > 1000 {
            println!("processed {}/{}", ordipool.len(), mpr_len);
            break;
        }
    }

    // count the inscriptions and bytes, group by media type
    let mut media_counts: HashMap<Media, usize> = HashMap::new();
    let mut media_bytes: HashMap<Media, usize> = HashMap::new();

    for inscription in ordipool.values().flatten() {
        let media = inscription.media();
        let bytes = inscription.content_length().unwrap_or_default();
        *media_counts.entry(media).or_insert(0) += 1;
        *media_bytes.entry(media).or_insert(0) += bytes;
    }

    let mempool_info: Vec<_> = media_counts
        .iter()
        .map(|(key, count)| {
            let bytes = media_bytes.get(key).unwrap_or(&0).to_owned();
            format!("{:?}: {} ({} bytes)", key, count, bytes)
        })
        .collect();
    let event = LiveEvent::MempoolInfo(mempool_info.join(" | "));
    _ = EVENT_CHANNEL.send(&event).await;

    dbg!(media_counts);
    if broadcast.len() > 4 {
        broadcast.resize(4, String::new());
    }

    // no new inscription, show something random
    if &broadcast.len() == &0 {
        let chosen = ordipool
            .iter()
            .filter(|entry: &(&String, &Option<Inscription>)| {
                entry.1.is_some() && entry.1.clone().unwrap().media() == Media::Image
            })
            .choose(&mut rand::thread_rng());
        if chosen.is_some() {
            let txid = chosen.unwrap().0;
            let event = LiveEvent::RandomInscription(format!("{}i0", &txid));
            _ = EVENT_CHANNEL.send(&event).await;
        }
    } else {
        for txid in &broadcast {
            let event = LiveEvent::NewInscription(format!("{}i0", &txid));
            _ = EVENT_CHANNEL.send(&event).await;
        }
    }

    log!(
        "tick: bitcoin_core, {}, {}, {}",
        &mpr_len,
        &broadcast.len(),
        &ordipool.len()
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

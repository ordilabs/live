use crate::types::{LiveEvent, MempoolAllInfo, MempoolInfo};

use super::*;
use std::collections::HashMap;
extern crate rand;

use rand::seq::IteratorRandom;

use ord_labs::{Inscription, Media};

use backend::BitcoinCore;
use serde::*;

use axum::http::{header, HeaderMap, StatusCode};
use axum::{extract::Path, response::IntoResponse};
//use std::io::Read::read_to_end;

#[derive(Deserialize)]
pub struct Content {
  pub inscription_id: String,
}

// #[tracing::instrument]
pub(crate) async fn tick_bitcoin_core(
  backend: &BitcoinCore,
  ordipool: &mut HashMap<String, Option<Inscription>>,
) {
  // Current block
  let block_count = backend.get_block_count().await;
  tracing::info!(?block_count);

  // TODO (@sectore) Remove it - just for testing serialization/deserialization LiveEvents (see #100)
  let t = std::time::SystemTime::now();
  let event = LiveEvent::ServerTime(t);
  _ = EVENT_CHANNEL.send(&event).await;

  let event = LiveEvent::BlockCount(block_count);
  _ = EVENT_CHANNEL.send(&event).await;

  let tick_start = std::time::Instant::now();

  // TODO(@sectore) Disable `recent` call temporarily - it breaks on mainnet
  // let mpr: MempoolRecent = Vec::default();
  let mpr = backend.recent().await.ok();
  let mpr = mpr.unwrap_or_default();

  let mpr_len = mpr.len();
  let mut broadcast = vec![];

  // remove all non-mempool txs,
  // current impl is slow and happens mostly when a new block was found
  let tick_retain = std::time::Instant::now();
  if ordipool.len() > mpr.len() {
    let txid: Vec<_> = mpr.iter().map(|entry| &entry.txid).collect();
    ordipool.retain(|key, _| txid.contains(&key));
  }
  let duration_retain = std::time::Instant::now() - tick_retain;

  // let mut backend_query_count = 0;

  for (_n, entry) in mpr.into_iter().enumerate() {
    let txid = entry.txid;
    if ordipool.contains_key(&txid) {
      continue;
    }

    let maybe_inscription = backend.maybe_inscription(&txid).await;
    // backend_query_count += 1;

    let maybe_inscription = maybe_inscription.unwrap_or_else(|e| {
      log::warn!("{}", e);
      None
    });

    ordipool.insert(txid.clone(), maybe_inscription.clone());

    // on startup, don't process too many per tick
    let duration = std::time::Instant::now() - tick_start;
    if duration.as_millis() > 1000 {
      println!("processed {}/{}", ordipool.len(), mpr_len);
      break;
    }

    if maybe_inscription.is_none() {
      continue;
    }

    let inscription = maybe_inscription.unwrap();
    // only broadcast images for now
    if inscription.media() != Media::Image {
      continue;
    }

    let inscription_id = format!("{}", &txid);
    broadcast.push(inscription_id);
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

  let mempool_info: MempoolAllInfo = media_counts
    .iter()
    .map(|(media, count)| {
      let bytes = media_bytes.get(media).unwrap_or(&0).to_owned();
      MempoolInfo {
        media: media.clone(),
        size: bytes,
        count: count.to_owned(),
      }
    })
    .collect();
  // TODO(sectore) For debugging only - will be removed in future
  // mempool_info.push(MempoolInfo {
  //   media: Media::Audio,
  //   size: 350000,
  //   count: 3,
  // });
  // mempool_info.push(MempoolInfo {
  //   media: Media::Pdf,
  //   size: 50000,
  //   count: 11,
  // });
  // mempool_info.push(MempoolInfo {
  //   media: Media::Iframe,
  //   size: 5000,
  //   count: 45,
  // });
  let event = LiveEvent::MempoolInfo(mempool_info);
  _ = EVENT_CHANNEL.send(&event).await;

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
  let duration = std::time::Instant::now() - tick_start;
  let format = num_format::CustomFormat::builder()
    .separator("_")
    .build()
    .unwrap();

  let mut buf = num_format::Buffer::new();
  buf.write_formatted(&duration.as_micros(), &format);
  let mut buf2 = num_format::Buffer::new();
  buf2.write_formatted(&duration_retain.as_micros(), &format);

  log!(
    "tick: bitcoin_core, {}, {}, {}, {}, {}",
    &mpr_len,
    &broadcast.len(),
    &ordipool.len(),
    buf.as_str(),
    buf2.as_str(),
  );
}

pub(crate) async fn content(
  Path(inscription_id): Path<String>,
  State(_client): State<HttpClient>,
  State(core): State<BitcoinCore>,
) -> impl IntoResponse {
  let s = inscription_id;

  let mut header_map = HeaderMap::new();

  //dbg!(&s);
  if s.starts_with("punk") {
    let location = format!("/punks/{}", s);

    header_map.insert(
      header::LOCATION,
      header::HeaderValue::from_str(&location).unwrap(),
    );
    return (StatusCode::TEMPORARY_REDIRECT, header_map, vec![]);

    //header_map.append(header::LOCATION, uri.parse().unwrap());
    //return (StatusCode::TEMPORARY_REDIRECT, header_map, vec![]);
  }

  if s.len() < 64 {
    return (
      StatusCode::NOT_FOUND,
      header_map,
      "Error 404: Not found".as_bytes().to_vec(),
    );
  }

  let txid = &s.as_str()[0..64];
  let index: Result<usize, _> = s[65..].parse();
  if index.is_err() {
    return (StatusCode::NOT_FOUND, header_map, vec![]);
  }

  let index = index.unwrap();

  let query = core.maybe_inscription_with_index(txid, index).await;

  if query.is_err() {
    return (
      StatusCode::INTERNAL_SERVER_ERROR,
      header_map,
      format!("{:?}", query.err()).as_bytes().to_vec(),
    );
  }

  let maybe_inscription = query.unwrap();

  if maybe_inscription.is_none() {
    return (
      StatusCode::NOT_FOUND,
      header_map,
      "Error 404: Not found".as_bytes().to_vec(),
    );
  };

  let inscription = maybe_inscription.unwrap();

  if inscription.media() != Media::Image {
    return (
      StatusCode::OK,
      header_map,
      "Not an image".as_bytes().to_vec(),
    );
  }

  return image_response_axum(inscription);
}

fn image_response_axum(inscription: Inscription) -> (StatusCode, HeaderMap, Vec<u8>) {
  let mut headers = HeaderMap::new();

  headers.insert(
    header::CONTENT_SECURITY_POLICY,
    "default-src 'unsafe-eval' 'unsafe-inline' data:"
      .parse()
      .unwrap(),
  );
  headers.insert(
    header::CACHE_CONTROL,
    "max-age=31536000, immutable".parse().unwrap(),
  );

  inscription
    .content_type()
    .map(|content_type| headers.insert(header::CONTENT_TYPE, content_type.parse().unwrap()));

  inscription
    .content_length()
    .map(|content_length| headers.insert(header::CONTENT_LENGTH, content_length.into()));

  (StatusCode::OK, headers, inscription.into_body().unwrap())
}

// #[tracing::instrument]
pub async fn preview(Path(inscription_id): Path<String>) -> impl IntoResponse {
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

  (
    StatusCode::OK,
    [(
      header::CONTENT_TYPE,
      header::HeaderValue::from_static(mime::TEXT_HTML_UTF_8.as_ref()),
    )],
    resp.replace("{}", &inscription_id),
  )
}

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

pub async fn content(path: web::Path<Content>) -> impl Responder {
    if &path.inscription_id.len() < &64usize {
        return HttpResponse::NotFound()
            .content_type("text/plain")
            .body("body");
    }

    let txid = &path.inscription_id[0..64];

    // get content from remote server
    // todo gfi: use the /raw api instead of /hex
    let hex = {
        reqwest::get(format!("http://mempool-ol.local/api/tx/{}/hex", txid))
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    };
    let data = hex::decode(&hex).unwrap();
    let transaction: Transaction = consensus::deserialize(&data).unwrap();
    let inscription = Inscription::from_transaction(&transaction).unwrap();

    if inscription.media() != Media::Image {
        return HttpResponse::Ok()
            .content_type("text/plain")
            .body("Not an image");
    }

    match inscription.media() {
        Media::Image => image_response_actix(inscription),
        _ => unreachable!("Only images are supported for now"),
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

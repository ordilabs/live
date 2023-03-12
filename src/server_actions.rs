use actix_web::*;
use bitcoin::blockdata::script::Instruction;
use bitcoin::*;
use hex::*;
use reqwest::*;
use serde::*;

//use std::io::Read::read_to_end;

#[derive(Deserialize)]
pub struct Content {
    pub inscription_id: String,
}

pub async fn content(path: web::Path<Content>) -> impl Responder {
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

    let tx: Transaction = consensus::deserialize(&data).unwrap();
    let reveal_script = tx.input[0].witness.second_to_last().unwrap().clone();
    let reveal_script = Script::from(reveal_script.to_vec());
    let asm = reveal_script.clone().asm();

    let r: Vec<_> = reveal_script
        .instructions()
        .into_iter()
        .filter_map(|i| match i.unwrap() {
            Instruction::PushBytes(d) => Some(d.to_owned()),
            Instruction::Op(_) => None,
        })
        .collect();

    HttpResponse::Ok()
        .content_type("image/webp")
        .body(r.last().unwrap().clone())
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

#![allow(dead_code)]
use std::*;

use leptos::*;

use std::time::SystemTime;

#[derive(Clone)]
pub(crate) struct StreamContext {
  pub inscription: ReadSignal<Option<String>>,
  pub info: ReadSignal<Option<String>>,
  pub block: ReadSignal<Option<u64>>,
  pub time: ReadSignal<Option<SystemTime>>,
}

pub fn provide_stream_context(cx: Scope) {
  if use_context::<StreamContext>(cx).is_none() {
    #[cfg(not(feature = "ssr"))]
    let context = {
      use futures::StreamExt;

      let mut source = gloo_net::eventsource::futures::EventSource::new("/api/events")
        .expect("couldn't connect to SSE stream");

      // Note:
      // All data of subscriptions are stringified JSON (similar to `JSON.stringify` in JS) - see implementation in `main.rs -> sse_handler`
      // And needs to be deserialized `serde_json::from_str` (similar to `JSON.parse` in JS)

      let inscription = create_signal_from_stream(
        cx,
        source.subscribe("inscription").unwrap().map(|value| {
          let s = value
            .expect("no message event")
            .1
            .data()
            .as_string()
            .expect("expected string value");

          serde_json::from_str::<String>(s.as_str()).expect("expected String value")
        }),
      );

      let info = create_signal_from_stream(
        cx,
        source.subscribe("info").unwrap().map(|value| {
          let s = value
            .expect("no message event")
            .1
            .data()
            .as_string()
            .expect("expected string value");

          serde_json::from_str::<String>(s.as_str()).expect("expected String value")
        }),
      );

      let block = create_signal_from_stream(
        cx,
        source.subscribe("block").unwrap().map(|value| {
          let s = value
            .expect("no message event")
            .1
            .data()
            .as_string()
            .expect("expected string value");
          serde_json::from_str::<u64>(s.as_str()).expect("expected u64 value")
        }),
      );

      // TODO (@sectore) Remove it - just for testing serialization/deserialization LiveEvents (see #100)
      let time = create_signal_from_stream(
        cx,
        source.subscribe("time").unwrap().map(|value| {
          let s = value
            .expect("no message event")
            .1
            .data()
            .as_string()
            .expect("expected string value");
          serde_json::from_str::<SystemTime>(s.as_str()).expect("expected SystemTime value")
        }),
      );

      on_cleanup(cx, move || source.close());

      StreamContext {
        inscription,
        info,
        block,
        time,
      }
    };

    #[cfg(feature = "ssr")]
    let context = StreamContext {
      inscription: create_signal(cx, None::<String>).0,
      info: create_signal(cx, None::<String>).0,
      block: create_signal(cx, None::<u64>).0,
      time: create_signal(cx, None::<SystemTime>).0,
    };

    provide_context(cx, context);
  }
}

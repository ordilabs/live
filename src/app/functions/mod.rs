pub mod inscription;
pub mod theme;

pub use inscription::*;
pub use theme::*;

use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "ssr")] {

    use leptos::ServerFn;

    pub fn register_server_functions() {
      _ = inscription::GetInscriptionDetails::register();
      _ = theme::SetDarkTheme::register();
    }
}}

use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "ssr")] {

    use leptos::*;

    #[server(GetLastInscription, "/api")]
    pub async fn get_last_inscription() -> Result<String, ServerFnError> {
      Ok("".to_string())
    }

    #[server(GetInscriptionDetails, "/api")]
    pub async fn get_inscription_details(id: String) -> Result<i32, ServerFnError> {
        Ok(42)
    }
  }
}

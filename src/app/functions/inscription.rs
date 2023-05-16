use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(feature = "ssr")] {

    use leptos::*;

    #[server(GetInscriptionDetails, "/api")]
    pub async fn get_inscription_details(_id: String) -> Result<i32, ServerFnError> {
        Ok(42)
    }
  }
}

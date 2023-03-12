
use leptos::*;
use leptos_router::{use_params_map};


#[component]
pub fn Stream(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div>
        "Stream"
        </div>
    }
}


// reruns whenever the :id param changes
async fn inscription_data(id: String) -> String {
  id
}

#[component]
pub fn Detail(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);

    let data = create_resource(
        cx,
        move || params.with(|p| p.get("id").cloned().unwrap_or_default()),
        move |id| inscription_data(id)
      );
    
    view! {
        cx,
        <div>
        "Detail"
        <span>{{move || params.with(|p| p.get("id").cloned().unwrap_or_default())}}</span>
        </div>        
    }
}
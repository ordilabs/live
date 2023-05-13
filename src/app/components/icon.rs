use leptos::*;

// https://heroicons.com/ -> audio
#[component]
pub fn IconAudio(cx: Scope, #[prop(optional)] class: &'static str) -> impl IntoView {
  view! { cx,
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class=format!("w-6 h-6 {class}")
      fill="none"
      viewBox="0 0 24 24"
      stroke-width="1.5"
      stroke="currentColor"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="M19.114 5.636a9 9 0 010 12.728M16.463 8.288a5.25 5.25 0 010 7.424M6.75 8.25l4.72-4.72a.75.75 0 011.28.53v15.88a.75.75 0 01-1.28.53l-4.72-4.72H4.51c-.88 0-1.704-.507-1.938-1.354A9.01 9.01 0 012.25 12c0-.83.112-1.633.322-2.396C2.806 8.756 3.63 8.25 4.51 8.25H6.75z"
      ></path>
    </svg>
  }
}

// https://heroicons.com/ -> video
#[component]
pub fn IconVideo(cx: Scope, #[prop(optional)] class: &'static str) -> impl IntoView {
  view! { cx,
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class=format!("w-6 h-6 {class}")
      fill="none"
      viewBox="0 0 24 24"
      stroke-width="1.5"
      stroke="currentColor"
    >
      <path
        stroke-linecap="round"
        d="M15.75 10.5l4.72-4.72a.75.75 0 011.28.53v11.38a.75.75 0 01-1.28.53l-4.72-4.72M4.5 18.75h9a2.25 2.25 0 002.25-2.25v-9a2.25 2.25 0 00-2.25-2.25h-9A2.25 2.25 0 002.25 7.5v9a2.25 2.25 0 002.25 2.25z"
      ></path>
    </svg>
  }
}

// https://heroicons.com/ -> image
#[component]
pub fn IconImage(cx: Scope, #[prop(optional)] class: &'static str) -> impl IntoView {
  view! { cx,
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class=format!("w-6 h-6 {class}")
      fill="none"
      viewBox="0 0 24 24"
      stroke-width="1.5"
      stroke="currentColor"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="M2.25 15.75l5.159-5.159a2.25 2.25 0 013.182 0l5.159 5.159m-1.5-1.5l1.409-1.409a2.25 2.25 0 013.182 0l2.909 2.909m-18 3.75h16.5a1.5 1.5 0 001.5-1.5V6a1.5 1.5 0 00-1.5-1.5H3.75A1.5 1.5 0 002.25 6v12a1.5 1.5 0 001.5 1.5zm10.5-11.25h.008v.008h-.008V8.25zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z"
      ></path>
    </svg>
  }
}

// https://heroicons.com/ -> question
#[component]
pub fn IconUnknown(cx: Scope, #[prop(optional)] class: &'static str) -> impl IntoView {
  view! { cx,
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class=format!("w-6 h-6 {class}")
      fill="none"
      viewBox="0 0 24 24"
      stroke-width="1.5"
      stroke="currentColor"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9 5.25h.008v.008H12v-.008z"
      ></path>
    </svg>
  }
}

// https://heroicons.com/ -> text
#[component]
pub fn IconText(cx: Scope, #[prop(optional)] class: &'static str) -> impl IntoView {
  view! { cx,
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class=format!("w-6 h-6 {class}")
      fill="none"
      viewBox="0 0 24 24"
      stroke-width="1.5"
      stroke="currentColor"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z"
      ></path>
    </svg>
  }
}

// https://solid-icons.vercel.app/search/pdf -> ImFilePdf
#[component]
pub fn IconPdf(cx: Scope, #[prop(optional)] class: &'static str) -> impl IntoView {
  view! { cx,
    <svg
      class=format!("w-4 h-4 overflow-visible {class}")
      fill="currentColor"
      stroke-width="0"
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 16 16"
    >
      <path d="M13.156 9.211c-.213-.21-.686-.321-1.406-.331a11.754 11.754 0 0 0-1.69.124c-.276-.159-.561-.333-.784-.542-.601-.561-1.103-1.34-1.415-2.197.02-.08.038-.15.054-.222 0 0 .339-1.923.249-2.573a.73.73 0 0 0-.044-.184l-.029-.076c-.092-.212-.273-.437-.556-.425l-.171-.005c-.316 0-.573.161-.64.403-.205.757.007 1.889.39 3.355l-.098.239c-.275.67-.619 1.345-.923 1.94l-.04.077c-.32.626-.61 1.157-.873 1.607l-.271.144c-.02.01-.485.257-.594.323-.926.553-1.539 1.18-1.641 1.678-.032.159-.008.362.156.456l.263.132a.792.792 0 0 0 .357.086c.659 0 1.425-.821 2.48-2.662a24.79 24.79 0 0 1 3.819-.908c.926.521 2.065.883 2.783.883.128 0 .238-.012.327-.036a.558.558 0 0 0 .325-.222c.139-.21.168-.499.13-.795a.531.531 0 0 0-.157-.271zM3.307 12.72c.12-.329.596-.979 1.3-1.556.044-.036.153-.138.253-.233-.736 1.174-1.229 1.642-1.553 1.788zm4.169-9.6c.212 0 .333.534.343 1.035s-.107.853-.252 1.113c-.12-.385-.179-.992-.179-1.389 0 0-.009-.759.088-.759zM6.232 9.961c.148-.264.301-.543.458-.839.383-.724.624-1.29.804-1.755a5.813 5.813 0 0 0 1.328 1.649c.065.055.135.111.207.166-1.066.211-1.987.467-2.798.779zm6.72-.06c-.065.041-.251.064-.37.064-.386 0-.864-.176-1.533-.464.257-.019.493-.029.705-.029.387 0 .502-.002.88.095s.383.293.318.333z"></path>
      <path d="M14.341 3.579c-.347-.473-.831-1.027-1.362-1.558S11.894 1.006 11.421.659C10.615.068 10.224 0 10 0H2.25C1.561 0 1 .561 1 1.25v13.5c0 .689.561 1.25 1.25 1.25h11.5c.689 0 1.25-.561 1.25-1.25V5c0-.224-.068-.615-.659-1.421zm-2.07-.85c.48.48.856.912 1.134 1.271h-2.406V1.595c.359.278.792.654 1.271 1.134zM14 14.75c0 .136-.114.25-.25.25H2.25a.253.253 0 0 1-.25-.25V1.25c0-.135.115-.25.25-.25H10v3.5a.5.5 0 0 0 .5.5H14v9.75z"></path>
    </svg>
  }
}

// https://solid-icons.vercel.app/search/pdf -> TbFrame
#[component]
pub fn IconIframe(cx: Scope, #[prop(optional)] class: &'static str) -> impl IntoView {
  view! { cx,
    <svg
      class=format!("w-6 h-6 {class}")
      viewBox="0 0 24 24"
      fill="none"
      stroke-width="2"
      xmlns="http://www.w3.org/2000/svg"
      stroke="currentColor"
      stroke-linecap="round"
      stroke-linejoin="round"
    >
      <path stroke="none" d="M0 0h24v24H0z"></path>
      <path d="M4 7h16M4 17h16M7 4v16M17 4v16"></path>
    </svg>
  }
}

// https://tailwindcss.com/docs/animation#spin
#[component]
pub fn IconLoader(cx: Scope, #[prop(optional)] class: &'static str) -> impl IntoView {
  view! { cx,
    <svg
      class=format!("h-5 w-5 animate-spin {class}")
      xmlns="http://www.w3.org/2000/svg"
      fill="none"
      viewBox="0 0 24 24"
    >
      <circle
        class="opacity-25"
        cx="12"
        cy="12"
        r="10"
        stroke="currentColor"
        stroke-width="4"
      ></circle>
      <path
        class="opacity-75"
        fill="currentColor"
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      ></path>
    </svg>
  }
}

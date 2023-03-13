use leptos::*;

#[allow(dead_code)]
static mut LOCAL_COUNT: i32 = 0;

#[allow(dead_code)]
pub fn incr() -> i32 {
    unsafe {
        LOCAL_COUNT += 1;
        LOCAL_COUNT
    }
}

#[component]
pub fn InscriptionGridItem(cx: Scope, inscription_id: Option<String>) -> impl IntoView {
    let initial = inscription_id.unwrap_or("https://images.unsplash.com/photo-1582053433976-25c00369fc93?ixid=MXwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHw%3D&ixlib=rb-1.2.1&auto=format&fit=crop&w=512&q=80".to_string());

    let (iframe_url, set_iframe_url) = create_signal(cx, initial);

    let click = move |_| {
        set_iframe_url.set(format!("/preview/punk_{}.webp", incr()));
    };

    log::debug!("hi from component");
    view! {
        cx,
        <li class="relative" on:click=click>
            // <!-- Current: "ring-2 ring-rose-500 ring-offset-2", Default: "focus-within:ring-2 focus-within:ring-rose-500 focus-within:ring-offset-2 focus-within:ring-offset-gray-100" -->
            <div
                class="ring-2 ring-rose-500 ring-offset-2 aspect-w-10 aspect-h-10 group block w-full overflow-hidden rounded-lg bg-gray-100">
                // <!-- Current: "", Default: "group-hover:opacity-75" -->
                <iframe sandbox="allow-scripts" scrolling="no" loading="lazy" class="pointer-events-none object-cover" src={iframe_url}></iframe>

                <button type="button" class="absolute inset-0 focus:outline-none">
                    <span class="sr-only">"View details for ordinal"</span>
                </button>
            </div>
            <p
                class="pointer-events-none mt-2 block truncate text-sm font-medium text-gray-900">
                "JPEG"</p>
            <p class="pointer-events-none block text-sm font-medium text-gray-500">"3.9 MB"</p>
        </li>
    }
}

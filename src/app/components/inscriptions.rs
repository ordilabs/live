use leptos::*;

#[component]
pub fn InscriptionGridItem(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <li class="relative">
            // <!-- Current: "ring-2 ring-indigo-500 ring-offset-2", Default: "focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 focus-within:ring-offset-gray-100" -->
            <div
                class="ring-2 ring-indigo-500 ring-offset-2 aspect-w-10 aspect-h-7 group block w-full overflow-hidden rounded-lg bg-gray-100">
                // <!-- Current: "", Default: "group-hover:opacity-75" -->
                <iframe sandbox="allow-scripts" scrolling="no" loading="lazy" class="pointer-events-none object-cover" src="https://images.unsplash.com/photo-1582053433976-25c00369fc93?ixid=MXwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHw%3D&ixlib=rb-1.2.1&auto=format&fit=crop&w=512&q=80"></iframe>
                
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

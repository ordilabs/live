use leptos::*;

#[component]    
pub fn DetailsSidebar(cx: Scope) -> impl IntoView {
    view! {cx,
        <aside class="hidden w-96 overflow-y-auto border-l border-gray-200 bg-white p-8 lg:block">
        <div class="space-y-6 pb-16">
            <div>
                <div class="aspect-w-10 aspect-h-7 block w-full overflow-hidden rounded-lg">
                    <img src="https://images.unsplash.com/photo-1582053433976-25c00369fc93?ixid=MXwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHw%3D&ixlib=rb-1.2.1&auto=format&fit=crop&w=512&q=80"
                        alt="" class="object-cover" />
                        <iframe sandbox="allow-scripts" scrolling="no" loading="lazy" class="object-cover" src="https://images.unsplash.com/photo-1582053433976-25c00369fc93?ixid=MXwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHw%3D&ixlib=rb-1.2.1&auto=format&fit=crop&w=512&q=80"></iframe>
                </div>
                <div class="mt-4 flex items-start justify-between">
                    <div>
                        <h2 class="text-lg font-medium text-gray-900"><span class="sr-only">"Details for"
                            </span>"JPEG"</h2>
                        <p class="text-sm font-medium text-gray-500">"3.9 MB"</p>
                    </div>
                </div>
            </div>
            <div>
                <dl class="mt-2 divide-y divide-gray-200 border-t border-b border-gray-200">
                    <div class="flex justify-between py-3 text-sm font-medium">
                        <dt class="text-gray-500">"Type"</dt>
                        <dd class="whitespace-nowrap text-gray-900">"Image/JPEG"</dd>
                    </div>
                    <div class="flex justify-between py-3 text-sm font-medium">
                        <dt class="text-gray-500">"Fee"</dt>
                        <dd class="whitespace-nowrap text-gray-900">"1234 sats"</dd>
                    </div>

                    <div class="flex justify-between py-3 text-sm font-medium">
                        <dt class="text-gray-500">"Fee rate"</dt>
                        <dd class="whitespace-nowrap text-gray-900">"4 sats/vB"</dd>
                    </div>

                    <div class="flex justify-between py-3 text-sm font-medium">
                        <dt class="text-gray-500">"Inscription TXID"</dt>
                    </div>
                    <div class="flex justify-between py-3 text-sm font-medium">

                        <dd class="whitespace-nowrap text-gray-900">
                            "0301E0480B374B32851A9462DB29DC19FE830A7F7D7A88B81612B9D42099C0AE"</dd>
                    </div>
                    <div class="flex justify-between py-3 text-sm font-medium">
                        <dt class="text-gray-500">"Inscriber"</dt>
                
                    </div>
                    <div class="flex justify-between py-3 text-sm font-medium">
                        <dd class="whitespace-nowrap text-gray-900">
                            "bc1pscu742m5eyt6vwzl62fjugy9mj5yq8pgk674qc2x44892t3zjqfs3ca78z"</dd>
                    </div>
                </dl>
            </div>
            <div class="flex gap-x-3">
                <button type="button"
                    class="flex-1 rounded-md bg-rose-600 py-2 px-3 text-sm font-semibold text-white shadow-sm hover:bg-rose-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-rose-600">"Share"</button>
                <button type="button"
                    class="flex-1 rounded-md bg-rose-600 py-2 px-3 text-sm font-semibold text-white shadow-sm hover:bg-rose-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-rose-600">"Download"</button>
            </div>
        </div>
    </aside>}
}
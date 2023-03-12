use leptos::*;
use leptos_router::*;
use leptos_meta::*;

#[cfg(feature = "ssr")]
use std::sync::atomic::{AtomicI32, Ordering};

#[cfg(feature = "ssr")]
use broadcaster::BroadcastChannel;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    _ = GetServerCount::register();
    _ = AdjustServerCount::register();
    _ = ClearServerCount::register();
}

#[cfg(feature = "ssr")]
static COUNT: AtomicI32 = AtomicI32::new(0);

#[cfg(feature = "ssr")]
lazy_static::lazy_static! {
    pub static ref COUNT_CHANNEL: BroadcastChannel<i32> = BroadcastChannel::new();
}
// "/api" is an optional prefix that allows you to locate server functions wherever you'd like on the server
#[server(GetServerCount, "/api")]
pub async fn get_server_count() -> Result<i32, ServerFnError> {
    Ok(COUNT.load(Ordering::Relaxed))
}

#[server(AdjustServerCount, "/api")]
pub async fn adjust_server_count(delta: i32, msg: String) -> Result<i32, ServerFnError> {
    let new = COUNT.load(Ordering::Relaxed) + delta;
    COUNT.store(new, Ordering::Relaxed);
    _ = COUNT_CHANNEL.send(&new).await;
    println!("message = {:?}", msg);
    Ok(new)
}

#[server(ClearServerCount, "/api")]
pub async fn clear_server_count() -> Result<i32, ServerFnError> {
    COUNT.store(0, Ordering::Relaxed);
    _ = COUNT_CHANNEL.send(&0).await;
    Ok(0)
}
#[component]
pub fn Ordilive(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    view! {
        cx,
        <Router> 
            <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
            <Stylesheet id="leptos" href="/pkg/ordilabs_live.css"/>

        <div class="flex h-full">
       // <!-- Narrow sidebar -->
        <NavSideBar />

    //     <!--
    //   Mobile menu
  
    //   Off-canvas menu for mobile, show/hide based on off-canvas menu state.
    // -->
   <MobileMenu />

        // <!-- Content area -->
        <div class="flex flex-1 flex-col overflow-hidden">
           <Header />

            // <!-- Main content -->
            <div class="flex flex-1 items-stretch overflow-hidden">
                <main class="flex-1 overflow-y-auto">
                    <div class="mx-auto max-w-7xl px-4 pt-8 sm:px-6 lg:px-8">
                        <div class="flex">
                            <h1 class="flex-1 text-2xl font-bold text-gray-900">"Stream"</h1>
                            <div class="ml-6 flex items-center rounded-lg bg-gray-100 p-0.5 sm:hidden">
                                <button type="button"
                                    class="rounded-md p-1.5 text-gray-400 hover:bg-white hover:shadow-sm focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500">
                                    <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                        <path fill-rule="evenodd"
                                            d="M2 3.75A.75.75 0 012.75 3h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 3.75zm0 4.167a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75a.75.75 0 01-.75-.75zm0 4.166a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75a.75.75 0 01-.75-.75zm0 4.167a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75a.75.75 0 01-.75-.75z"
                                            clip-rule="evenodd" />
                                    </svg>
                                    <span class="sr-only">"Use list view"</span>
                                </button>
                                <button type="button"
                                    class="ml-0.5 rounded-md bg-white p-1.5 text-gray-400 shadow-sm focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500">
                                    <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                        <path fill-rule="evenodd"
                                            d="M4.25 2A2.25 2.25 0 002 4.25v2.5A2.25 2.25 0 004.25 9h2.5A2.25 2.25 0 009 6.75v-2.5A2.25 2.25 0 006.75 2h-2.5zm0 9A2.25 2.25 0 002 13.25v2.5A2.25 2.25 0 004.25 18h2.5A2.25 2.25 0 009 15.75v-2.5A2.25 2.25 0 006.75 11h-2.5zm9-9A2.25 2.25 0 0011 4.25v2.5A2.25 2.25 0 0013.25 9h2.5A2.25 2.25 0 0018 6.75v-2.5A2.25 2.25 0 0015.75 2h-2.5zm0 9A2.25 2.25 0 0011 13.25v2.5A2.25 2.25 0 0013.25 18h2.5A2.25 2.25 0 0018 15.75v-2.5A2.25 2.25 0 0015.75 11h-2.5z"
                                            clip-rule="evenodd" />
                                    </svg>
                                    <span class="sr-only">"Use grid view"</span>
                                </button>
                            </div>
                        </div>

                        // <!-- Tabs -->
                        <TypeTabs />
                        // <!-- Gallery -->
                        <section class="mt-8 pb-16" aria-labelledby="gallery-heading">
                            <h2 id="gallery-heading" class="sr-only">"Recently viewed"</h2>
                            <ul role="list"
                                class="grid grid-cols-2 gap-x-4 gap-y-8 sm:grid-cols-3 sm:gap-x-6 md:grid-cols-4 lg:grid-cols-3 xl:grid-cols-4 xl:gap-x-8">
                                <InscriptionGridItem />
                                <InscriptionGridItem /><InscriptionGridItem />

                                // <!-- More files... -->

                                <Routes>
                                        <Route path="" view=|cx| view! {
                                            cx,
                                            <Stream/>
                                        }/>
                                        <Route path="tx" view=|cx| view! {
                                            cx,
                                            <Detail/>
                                        }/>
       
                                    </Routes>
                            </ul>
                        </section>
                    </div>
                </main>

                // <!-- Details sidebar -->
                <DetailsSidebar/>
            </div>
        </div>
    </div>
        </Router>
    }
}

#[component]
pub fn Stream(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div>
        "Stream"
        </div>
    }
}



#[component]
pub fn Detail(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div>
        "Detail"
        </div>
    }
}


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


#[component]
pub fn TypeTabs(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="mt-3 sm:mt-2">
                            <div class="sm:hidden">
                                <label for="tabs" class="sr-only">"Select a tab"</label>
                                // <!-- Use an "onChange" listener to redirect the user to the selected tab URL. -->
                                <select id="tabs" name="tabs"
                                    class="block w-full rounded-md border-0 py-1.5 pl-3 pr-10 text-gray-900 ring-1 ring-inset ring-gray-300 focus:border-indigo-500 focus:ring-2 focus:ring-inset focus:ring-indigo-600">
                                    <option selected>"All"</option>
                                    <option>"Images"</option>
                                    <option>"Names"</option>
                                </select>
                            </div>
                            <div class="hidden sm:block">
                                <div class="flex items-center border-b border-gray-200">
                                    <nav class="-mb-px flex flex-1 space-x-6 xl:space-x-8" aria-label="Tabs">
                                        // <!-- Current: "border-indigo-500 text-indigo-600", Default: "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700" -->
                                        <a href="#" aria-current="page"
                                            class="border-indigo-500 text-indigo-600 whitespace-nowrap border-b-2 py-4 px-1 text-sm font-medium">"All"</a>

                                        <a href="#"
                                            class="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 whitespace-nowrap border-b-2 py-4 px-1 text-sm font-medium">"Images"</a>

                                        <a href="#"
                                            class="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 whitespace-nowrap border-b-2 py-4 px-1 text-sm font-medium">"Names"</a>
                                    </nav>
                                    <div class="ml-6 hidden items-center rounded-lg bg-gray-100 p-0.5 sm:flex">
                                        <button type="button"
                                            class="rounded-md p-1.5 text-gray-400 hover:bg-white hover:shadow-sm focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500">
                                            <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor"
                                                aria-hidden="true">
                                                <path fill-rule="evenodd"
                                                    d="M2 3.75A.75.75 0 012.75 3h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 3.75zm0 4.167a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75a.75.75 0 01-.75-.75zm0 4.166a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75a.75.75 0 01-.75-.75zm0 4.167a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75a.75.75 0 01-.75-.75z"
                                                    clip-rule="evenodd" />
                                            </svg>
                                            <span class="sr-only">"Use list view"</span>
                                        </button>
                                        <button type="button"
                                            class="ml-0.5 rounded-md bg-white p-1.5 text-gray-400 shadow-sm focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500">
                                            <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor"
                                                aria-hidden="true">
                                                <path fill-rule="evenodd"
                                                    d="M4.25 2A2.25 2.25 0 002 4.25v2.5A2.25 2.25 0 004.25 9h2.5A2.25 2.25 0 009 6.75v-2.5A2.25 2.25 0 006.75 2h-2.5zm0 9A2.25 2.25 0 002 13.25v2.5A2.25 2.25 0 004.25 18h2.5A2.25 2.25 0 009 15.75v-2.5A2.25 2.25 0 006.75 11h-2.5zm9-9A2.25 2.25 0 0011 4.25v2.5A2.25 2.25 0 0013.25 9h2.5A2.25 2.25 0 0018 6.75v-2.5A2.25 2.25 0 0015.75 2h-2.5zm0 9A2.25 2.25 0 0011 13.25v2.5A2.25 2.25 0 0013.25 18h2.5A2.25 2.25 0 0018 15.75v-2.5A2.25 2.25 0 0015.75 11h-2.5z"
                                                    clip-rule="evenodd" />
                                            </svg>
                                            <span class="sr-only">"Use grid view"</span>
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
    }
}

#[component]    
pub fn Header(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <header class="w-full">
        <div class="relative z-10 flex h-16 flex-shrink-0 border-b border-gray-200 bg-white shadow-sm">
            <button type="button"
                class="border-r border-gray-200 px-4 text-gray-500 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500 md:hidden">
                <span class="sr-only">"Open sidebar"</span>
                <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor"
                    aria-hidden="true">
                    <path stroke-linecap="round" stroke-linejoin="round"
                        d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25H12" />
                </svg>
            </button>
            <div class="flex flex-1 justify-between px-4 sm:px-6">
                <div class="flex flex-1">
                    <form class="flex w-full md:ml-0" action="#" method="GET">
                        <label for="desktop-search-field" class="sr-only">"Search mempool ordinals"</label>
                        <label for="mobile-search-field" class="sr-only">"Search mempool ordinals"</label>
                        <div class="relative w-full text-gray-400 focus-within:text-gray-600">
                            <div class="pointer-events-none absolute inset-y-0 left-0 flex items-center">
                                <svg class="h-5 w-5 flex-shrink-0" viewBox="0 0 20 20" fill="currentColor"
                                    aria-hidden="true">
                                    <path fill-rule="evenodd"
                                        d="M9 3.5a5.5 5.5 0 100 11 5.5 5.5 0 000-11zM2 9a7 7 0 1112.452 4.391l3.328 3.329a.75.75 0 11-1.06 1.06l-3.329-3.328A7 7 0 012 9z"
                                        clip-rule="evenodd" />
                                </svg>
                            </div>
                            <input name="mobile-search-field" id="mobile-search-field"
                                class="h-full w-full border-0 py-2 pl-8 pr-3 text-base text-gray-900 focus:outline-none focus:ring-0 focus:placeholder:text-gray-400 sm:hidden"
                                placeholder="Search" type="search" />
                            <input name="desktop-search-field" id="desktop-search-field"
                                class="hidden h-full w-full border-0 py-2 pl-8 pr-3 text-sm text-gray-900 focus:outline-none focus:ring-0 focus:placeholder:text-gray-400 sm:block"
                                placeholder="Search mempool ordinals" type="search" />
                        </div>
                    </form>
                </div>
                <div class="ml-2 flex items-center space-x-4 sm:ml-6 sm:space-x-6">

                </div>
            </div>
        </div>
    </header>
}
}
#[component]
pub fn NavSideBar(cx: Scope) -> impl IntoView {
    view! {
        cx,
              <div class="hidden w-28 overflow-y-auto bg-indigo-700 md:block">
                            // <!-- Narrow sidebar -->

              <div class="flex w-full flex-col items-center py-6">
                  <div class="flex flex-shrink-0 items-center">
                      <img class="h-8 w-auto" src="https://tailwindui.com/img/logos/mark.svg?color=white"
                          alt="Your Company" />
                  </div>
                  <div class="mt-6 w-full flex-1 space-y-1 px-2">
            //          <!-- Current: "bg-indigo-800 text-white", Default: "text-indigo-100 hover:bg-indigo-800 hover:text-white" -->
  
  
                      <a href="#"
                          class="bg-indigo-800 text-white group flex w-full flex-col items-center rounded-md p-3 text-xs font-medium"
                          aria-current="page">
                          <svg class="text-white h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                              stroke="currentColor" aria-hidden="true">
                              <path stroke-linecap="round" stroke-linejoin="round"
                                  d="M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25a2.25 2.25 0 01-2.25-2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z" />
                          </svg>
                          <span class="mt-2">"Stream"</span>
                      </a>
  
                      <a href="#"
                      class="text-indigo-100 hover:bg-indigo-800 hover:text-white group flex w-full flex-col items-center rounded-md p-3 text-xs font-medium"
                      aria-current="page">
                      <svg class="text-white h-6 w-6"  fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                          <path stroke-linecap="round" stroke-linejoin="round" d="M12 6v12m-3-2.818l.879.659c1.171.879 3.07.879 4.242 0 1.172-.879 1.172-2.303 0-3.182C13.536 12.219 12.768 12 12 12c-.725 0-1.45-.22-2.003-.659-1.106-.879-1.106-2.303 0-3.182s2.9-.879 4.006 0l.415.33M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                        </svg>
                      <span class="mt-2">"Whales"</span>
                  </a>
  
                  <a href="#"
                  class="text-indigo-100 hover:bg-indigo-800 hover:text-white group flex w-full flex-col items-center rounded-md p-3 text-xs font-medium"
                  aria-current="page">
                  <svg class="text-white h-6 w-6"  fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                      <path stroke-linecap="round" stroke-linejoin="round" d="M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                    </svg>
                  
                  <span class="mt-2">"Plebs"</span>
              </a>
  
  
                  </div>
              </div>
          </div>
    }

}

#[component]    
pub fn MobileMenu(cx: Scope) -> impl IntoView {
    view! {cx, 
        <div class="relative z-40 md:hidden" role="dialog" aria-modal="true">
        //         <!--
        //     Off-canvas menu backdrop, show/hide based on off-canvas menu state.
      
        //     Entering: "transition-opacity ease-linear duration-300"
        //       From: "opacity-0"
        //       To: "opacity-100"
        //     Leaving: "transition-opacity ease-linear duration-300"
        //       From: "opacity-100"
        //       To: "opacity-0"
        //   -->
               <div class="fixed inset-0 bg-gray-600 bg-opacity-75"></div>
    
                <div class="fixed inset-0 z-40 flex">
            //         <!--
            //   Off-canvas menu, show/hide based on off-canvas menu state.
      
            //   Entering: "transition ease-in-out duration-300 transform"
            //     From: "-translate-x-full"
            //     To: "translate-x-0"
            //   Leaving: "transition ease-in-out duration-300 transform"
            //     From: "translate-x-0"
            //     To: "-translate-x-full"
            // -->
                    <div class="relative flex w-full max-w-xs flex-1 flex-col bg-indigo-700 pt-5 pb-4">
            //             <!--
            //     Close button, show/hide based on off-canvas menu state.
      
            //     Entering: "ease-in-out duration-300"
            //       From: "opacity-0"
            //       To: "opacity-100"
            //     Leaving: "ease-in-out duration-300"
            //       From: "opacity-100"
            //       To: "opacity-0"
            //   -->
                        <div class="absolute top-1 right-0 -mr-14 p-1">
                            <button type="button"
                                class="flex h-12 w-12 items-center justify-center rounded-full focus:outline-none focus:ring-2 focus:ring-white">
                                <svg class="h-6 w-6 text-white" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                                    stroke="currentColor" aria-hidden="true">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                </svg>
                                <span class="sr-only">"Close sidebar"</span>
                            </button>
                        </div>
    
                        <div class="flex flex-shrink-0 items-center px-4">
                            <img class="h-8 w-auto" src="https://tailwindui.com/img/logos/mark.svg?color=white"
                                alt="Your Company" />
                        </div>
                        <div class="mt-5 h-0 flex-1 overflow-y-auto px-2">
                            <nav class="flex h-full flex-col">
                                <div class="space-y-1">
                                    // <!-- Current: "bg-indigo-800 text-white", Default: "text-indigo-100 hover:bg-indigo-800 hover:text-white" -->
                       
    
                                    <a href="#"
                                        class="bg-indigo-800 text-white group flex items-center rounded-md py-2 px-3 text-sm font-medium"
                                        aria-current="page">
                                        <svg class="text-indigo-300 group-hover:text-white mr-3 h-6 w-6" fill="none"
                                        viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                                        <path stroke-linecap="round" stroke-linejoin="round"
                                            d="M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25a2.25 2.25 0 01-2.25-2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z" />
                                    </svg>
                                        <span>"Stream"</span>
                                    </a>
    
                                    <a href="#"
                                    class="bg-indigo-800 text-white group flex items-center rounded-md py-2 px-3 text-sm font-medium"
                                    aria-current="page">
                                    <svg class="text-indigo-300 group-hover:text-white mr-3 h-6 w-6"  fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 6v12m-3-2.818l.879.659c1.171.879 3.07.879 4.242 0 1.172-.879 1.172-2.303 0-3.182C13.536 12.219 12.768 12 12 12c-.725 0-1.45-.22-2.003-.659-1.106-.879-1.106-2.303 0-3.182s2.9-.879 4.006 0l.415.33M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                      </svg>
                                   
                                    <span>"Whales"</span>
                                </a>
                                <a href="#"
                                class="bg-indigo-800 text-white group flex items-center rounded-md py-2 px-3 text-sm font-medium"
                                aria-current="page">
                                <svg  class="text-indigo-300 group-hover:text-white mr-3 h-6 w-6" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                  </svg>
                               
                                <span>"Plebs"</span>
                            </a>
    
                                  
                                </div>
                            </nav>
                        </div>
                    </div>
    
                    <div class="w-14 flex-shrink-0" aria-hidden="true">
                        // <!-- Dummy element to force sidebar to shrink to fit close icon -->
                    </div>
                </div>
            </div>
    }
}

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
                    class="flex-1 rounded-md bg-indigo-600 py-2 px-3 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">"Share"</button>
                <button type="button"
                    class="flex-1 rounded-md bg-indigo-600 py-2 px-3 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">"Download"</button>
            </div>
        </div>
    </aside>}
}


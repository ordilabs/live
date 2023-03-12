use leptos::*;
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
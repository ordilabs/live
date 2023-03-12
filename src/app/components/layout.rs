use leptos::*;
#[component]    
pub fn Header(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <header class="w-full">
        <nav class="bg-gray-800">
  <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
    <div class="flex h-16 justify-between">
      <div class="flex">
        <div class="-ml-2 mr-2 flex items-center md:hidden">
        </div>
        <div class="flex flex-shrink-0 items-center">
          <img class="block h-8 w-auto lg:hidden" src="https://tailwindui.com/img/logos/mark.svg?color=rose&shade=500" alt="Your Company"/>
          <img class="hidden h-8 w-auto lg:block" src="https://tailwindui.com/img/logos/mark.svg?color=rose&shade=500" alt="Your Company"/>
        </div>
        <div class="hidden md:ml-6 md:flex md:items-center md:space-x-4">
     
        </div>
      </div>
      <div class="flex items-center">
        <div class="flex-shrink-0">
          <button type="button" class="relative inline-flex items-center gap-x-1.5 rounded-md bg-rose-500 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-rose-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-rose-500">

            <svg class="-ml-0.5 h-5 w-5" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
  <path stroke-linecap="round" stroke-linejoin="round" d="M21.75 6.75v10.5a2.25 2.25 0 01-2.25 2.25h-15a2.25 2.25 0 01-2.25-2.25V6.75m19.5 0A2.25 2.25 0 0019.5 4.5h-15a2.25 2.25 0 00-2.25 2.25m19.5 0v.243a2.25 2.25 0 01-1.07 1.916l-7.5 4.615a2.25 2.25 0 01-2.36 0L3.32 8.91a2.25 2.25 0 01-1.07-1.916V6.75"></path>
</svg>
            "Get updates"
          </button>
        </div>
        <div class="hidden md:ml-4 md:flex md:flex-shrink-0 md:items-center">


        //   <!-- Profile dropdown -->
          <div class="relative ml-3">

          </div>
        </div>
      </div>
    </div>
  </div>
</nav>

    </header>
}
}

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! {
        cx,
      
        <footer class="bg-white">
        <div class="mx-auto max-w-7xl py-12 px-6 md:flex md:items-center md:justify-between lg:px-8">
          <div class="flex justify-center space-x-6 md:order-2">
          
      
            <a href="#" class="text-gray-400 hover:text-gray-500">
              <span class="sr-only">"Twitter"</span>
              <svg class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path d="M8.29 20.251c7.547 0 11.675-6.253 11.675-11.675 0-.178 0-.355-.012-.53A8.348 8.348 0 0022 5.92a8.19 8.19 0 01-2.357.646 4.118 4.118 0 001.804-2.27 8.224 8.224 0 01-2.605.996 4.107 4.107 0 00-6.993 3.743 11.65 11.65 0 01-8.457-4.287 4.106 4.106 0 001.27 5.477A4.072 4.072 0 012.8 9.713v.052a4.105 4.105 0 003.292 4.022 4.095 4.095 0 01-1.853.07 4.108 4.108 0 003.834 2.85A8.233 8.233 0 012 18.407a11.616 11.616 0 006.29 1.84" />
              </svg>
            </a>
      
            <a href="https://github.com/ordilabs" class="text-gray-400 hover:text-gray-500">
              <span class="sr-only">"GitHub"</span>
              <svg class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd" />
              </svg>
            </a>
      
           
          </div>
          <div class="mt-8 md:order-1 md:mt-0">
            <p class="text-center text-xs leading-5 text-gray-500">"&copy; 2023 Ordilabs."</p>
          </div>
        </div>
      </footer>
      

    }
}
#[component]
pub fn NavSideBar(cx: Scope) -> impl IntoView {
    view! {
        cx,
              <div class="hidden w-28 overflow-y-auto bg-rose-700 md:block">
                            // <!-- Narrow sidebar -->

              <div class="flex w-full flex-col items-center py-6">
                  <div class="flex flex-shrink-0 items-center">
                      <img class="h-8 w-auto" src="https://tailwindui.com/img/logos/mark.svg?color=white"
                          alt="Your Company" />
                  </div>
                  <div class="mt-6 w-full flex-1 space-y-1 px-2">
            //          <!-- Current: "bg-rose-800 text-white", Default: "text-rose-100 hover:bg-rose-800 hover:text-white" -->
  
  
                      <a href="#"
                          class="bg-rose-800 text-white group flex w-full flex-col items-center rounded-md p-3 text-xs font-medium"
                          aria-current="page">
                          <svg class="text-white h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                              stroke="currentColor" aria-hidden="true">
                              <path stroke-linecap="round" stroke-linejoin="round"
                                  d="M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25a2.25 2.25 0 01-2.25-2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z" />
                          </svg>
                          <span class="mt-2">"Stream"</span>
                      </a>
  
                      <a href="#"
                      class="text-rose-100 hover:bg-rose-800 hover:text-white group flex w-full flex-col items-center rounded-md p-3 text-xs font-medium"
                      aria-current="page">
                      <svg class="text-white h-6 w-6"  fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                          <path stroke-linecap="round" stroke-linejoin="round" d="M12 6v12m-3-2.818l.879.659c1.171.879 3.07.879 4.242 0 1.172-.879 1.172-2.303 0-3.182C13.536 12.219 12.768 12 12 12c-.725 0-1.45-.22-2.003-.659-1.106-.879-1.106-2.303 0-3.182s2.9-.879 4.006 0l.415.33M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                        </svg>
                      <span class="mt-2">"Whales"</span>
                  </a>
  
                  <a href="#"
                  class="text-rose-100 hover:bg-rose-800 hover:text-white group flex w-full flex-col items-center rounded-md p-3 text-xs font-medium"
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
                    <div class="relative flex w-full max-w-xs flex-1 flex-col bg-rose-700 pt-5 pb-4">
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
                                    // <!-- Current: "bg-rose-800 text-white", Default: "text-rose-100 hover:bg-rose-800 hover:text-white" -->
                       
    
                                    <a href="#"
                                        class="bg-rose-800 text-white group flex items-center rounded-md py-2 px-3 text-sm font-medium"
                                        aria-current="page">
                                        <svg class="text-rose-300 group-hover:text-white mr-3 h-6 w-6" fill="none"
                                        viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true">
                                        <path stroke-linecap="round" stroke-linejoin="round"
                                            d="M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25a2.25 2.25 0 01-2.25-2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z" />
                                    </svg>
                                        <span>"Stream"</span>
                                    </a>
    
                                    <a href="#"
                                    class="bg-rose-800 text-white group flex items-center rounded-md py-2 px-3 text-sm font-medium"
                                    aria-current="page">
                                    <svg class="text-rose-300 group-hover:text-white mr-3 h-6 w-6"  fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 6v12m-3-2.818l.879.659c1.171.879 3.07.879 4.242 0 1.172-.879 1.172-2.303 0-3.182C13.536 12.219 12.768 12 12 12c-.725 0-1.45-.22-2.003-.659-1.106-.879-1.106-2.303 0-3.182s2.9-.879 4.006 0l.415.33M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                      </svg>
                                   
                                    <span>"Whales"</span>
                                </a>
                                <a href="#"
                                class="bg-rose-800 text-white group flex items-center rounded-md py-2 px-3 text-sm font-medium"
                                aria-current="page">
                                <svg  class="text-rose-300 group-hover:text-white mr-3 h-6 w-6" fill="none" stroke="currentColor" stroke-width="1.5" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
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
                                    class="block w-full rounded-md border-0 py-1.5 pl-3 pr-10 text-gray-900 ring-1 ring-inset ring-gray-300 focus:border-rose-500 focus:ring-2 focus:ring-inset focus:ring-rose-600">
                                    <option selected>"All"</option>
                                    <option>"Images"</option>
                                    <option>"Names"</option>
                                </select>
                            </div>
                            <div class="hidden sm:block">
                                <div class="flex items-center border-b border-gray-200">
                                    <nav class="-mb-px flex flex-1 space-x-6 xl:space-x-8" aria-label="Tabs">
                                        // <!-- Current: "border-rose-500 text-rose-600", Default: "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700" -->
                                        <a href="#" aria-current="page"
                                            class="border-rose-500 text-rose-600 whitespace-nowrap border-b-2 py-4 px-1 text-sm font-medium">"All"</a>

                                        <a href="#"
                                            class="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 whitespace-nowrap border-b-2 py-4 px-1 text-sm font-medium">"Images"</a>

                                        <a href="#"
                                            class="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 whitespace-nowrap border-b-2 py-4 px-1 text-sm font-medium">"Names"</a>
                                    </nav>
                                    <div class="ml-6 hidden items-center rounded-lg bg-gray-100 p-0.5 sm:flex">
                                        <button type="button"
                                            class="rounded-md p-1.5 text-gray-400 hover:bg-white hover:shadow-sm focus:outline-none focus:ring-2 focus:ring-inset focus:ring-rose-500">
                                            <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor"
                                                aria-hidden="true">
                                                <path fill-rule="evenodd"
                                                    d="M2 3.75A.75.75 0 012.75 3h14.5a.75.75 0 010 1.5H2.75A.75.75 0 012 3.75zm0 4.167a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75a.75.75 0 01-.75-.75zm0 4.166a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75a.75.75 0 01-.75-.75zm0 4.167a.75.75 0 01.75-.75h14.5a.75.75 0 010 1.5H2.75a.75.75 0 01-.75-.75z"
                                                    clip-rule="evenodd" />
                                            </svg>
                                            <span class="sr-only">"Use list view"</span>
                                        </button>
                                        <button type="button"
                                            class="ml-0.5 rounded-md bg-white p-1.5 text-gray-400 shadow-sm focus:outline-none focus:ring-2 focus:ring-inset focus:ring-rose-500">
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
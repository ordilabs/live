use crate::app::components::preview::*;

use leptos::*;
use leptos_router::*;

#[component]
pub fn Container(cx: Scope, children: Children) -> impl IntoView {
  view! { cx,
    <div class="pt-2 pb-2 lg:pt-3 lg:pb-4 px-4 bg-white even:bg-gray-100 dark:bg-slate-800 dark:even:bg-slate-900 ">
      {children(cx)}
    </div>
  }
}

#[component]
pub fn Title(cx: Scope, children: Children) -> impl IntoView {
  view! { cx, <h3 class="text-xs lg:text-md text-gray-400 dark:text-gray-400">{children(cx)}</h3> }
}

#[component]
pub fn Label(cx: Scope, children: Children) -> impl IntoView {
  view! { cx,
    <h2 class="text-xs sm:text-base lg:text-2xl text-gray-800 dark:text-gray-200 text-ellipsis overflow-hidden">
      {children(cx)}
    </h2>
  }
}

#[component]
pub fn Inscription(cx: Scope) -> impl IntoView {
  let params = use_params_map(cx);
  let id = move || params().get("id").cloned().unwrap_or_default();
  let hash = id.derive_signal(cx);
  let (fullscreen, set_fullscreen) = create_signal(cx, false);

  let isDev = std::env::var("LEPTOS_ENV")
    .unwrap_or("DEV".to_string())
    .to_lowercase()
    == "dev";

  view! { cx,
    <div class="content">
      {move || {
          match &id().is_empty() {
              true => {
                  view! { cx, <h1 class="text-4xl text-center">"Invalid inscription id"</h1> }
                      .into_view(cx)
              }
              false => {
                  view! { cx,
                    <div class="relative px-20 md:px-40 lg:px-60 my-10 flex items-center justify-center">
                      <button
                        class="group relative w-full h-full"
                        on:click=move |_| set_fullscreen.set(true)
                      >
                        <Preview
                          class="ring-4 sm:ring-8 ring-red-500 rounded-lg aspect-w-4 aspect-h-4 group-hover:brightness-100"
                          hash
                        />
                        <div class="absolute right-2 top-2 sm:right-5 sm:top-5 shadow-md ease
                        bg-white opacity-80 group-hover:opacity-100 text-gray-900 w-6 h-6 sm:w-10 sm:h-10 flex justify-center items-center p-1 sm:p-2 rounded-full">
                          <svg
                            xmlns="http://www.w3.org/2000/svg"
                            fill="none"
                            viewBox="0 0 24 24"
                            strokeWidth=1.5
                            stroke="currentColor"
                            class="w-6 h-6"
                          >
                            <path
                              strokeLinecap="round"
                              strokeLinejoin="round"
                              d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z"
                            ></path>
                          </svg>
                        </div>
                      </button>
                    </div>
                    <Container>
                      <Title>"id"</Title>
                      <Label>{&id()}</Label>
                    </Container>
                    {if isDev {
                        view! { cx,
                          <Container>
                            <Title>"address"</Title>
                            <Label>"bc1pz4kvfpurqc2hwgrq0nwtfve2lfxvdpfcdpzc6ujchyr3ztj6gd9sfr6ayf"</Label>
                          </Container>
                          <Container>
                            <Title>"output value"</Title>
                            <Label>"10000"</Label>
                          </Container>
                          <Container>
                            <Title>"sat"</Title>
                            <Label>"1252201400444387"</Label>
                          </Container>
                          <Container>
                            <Title>"content length"</Title>
                            <Label>"793 bytes"</Label>
                          </Container>
                          <Container>
                            <Title>"content type"</Title>
                            <Label>"image/png"</Label>
                          </Container>
                          <Container>
                            <Title>"time stamp"</Title>
                            <Label>"2022-12-14 20:32:00 UTC"</Label>
                          </Container>
                          <Container>
                            <Title>"genesis height"</Title>
                            <Label>"767430"</Label>
                          </Container>
                          <Container>
                            <Title>"genesis fee"</Title>
                            <Label>"322"</Label>
                          </Container>
                          <Container>
                            <Title>"genesis transaction"</Title>
                            <Label>"6fb976ab49dcec017f1e201e84395983204ae1a7c2abf7ced0a85d692e442799"</Label>
                          </Container>
                          <Container>
                            <Title>"location"</Title>
                            <Label>"bc4c30829a9564c0d58e6287195622b53ced54a25711d1b86be7cd3a70ef61ed:0:0"</Label>
                          </Container>
                          <Container>
                            <Title>"output"</Title>
                            <Label>"bc4c30829a9564c0d58e6287195622b53ced54a25711d1b86be7cd3a70ef61ed:0"</Label>
                          </Container>
                          <Container>
                            <Title>"offset"</Title>
                            <Label>"0"</Label>
                          </Container>
                        }
                    } else {
                        view! { cx, <></> }
                    }}
                  }
                      .into_view(cx)
              }
          }
      }}
      <A
        href="/"
        class="flex justify-center items-center ease text-gray-400 hover:text-gray-500 dark:text-gray-300 dark:hover:text-gray-200 p-4 my-5 text-md lg:text-xl"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          class="w-6 h-6 mr-1"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M11.25 9l-3 3m0 0l3 3m-3-3h7.5M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
          ></path>
        </svg>
        "Back"
      </A>
      <Show
        when=move || fullscreen()
        fallback=|cx| {
            view! { cx, <></> }
                .into_view(cx)
        }
      >
        <button class="group absolute w-full inset-0" on:click=move |_| set_fullscreen.set(false)>
          <Preview class="absolute inset-0 aspect-w-4 aspect-h-4" hash/>
          <div class="absolute right-4 top-4 p-2 ease bg-white opacity-80 group-hover:opacity-100 shadow-md rounded-full text-gray-600">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
              class="w-6 h-6"
            >
              <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"></path>
            </svg>
          </div>
        </button>
      </Show>
    </div>
  }
}
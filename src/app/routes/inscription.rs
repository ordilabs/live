use crate::app::components::preview::*;

use leptos::*;
use leptos_router::*;

#[component]
pub fn Container(cx: Scope, children: Children) -> impl IntoView {
  view! { cx,
    <div class="pt-2 pb-2 lg:pt-3 lg:pb-4 px-4 bg-white even:bg-gray-100 dark:bg-slate-800 dark:even:bg-slate-900 ">{children(cx)}</div>
  }
}

#[component]
pub fn Title(cx: Scope, children: Children) -> impl IntoView {
  view! { cx,
    <h3 class="text-xs lg:text-md text-gray-400 dark:text-gray-400">{children(cx)}</h3>
  }
}

#[component]
pub fn Label(cx: Scope, children: Children) -> impl IntoView {
  view! { cx,
    <h2 class="text-xs sm:text-base lg:text-2xl text-gray-800 dark:text-gray-200 text-ellipsis overflow-hidden">{children(cx)}</h2>
  }
}

#[component]
pub fn Inscription(cx: Scope) -> impl IntoView {
  let params = use_params_map(cx);
  let id = move || params().get("id").cloned().unwrap_or_default();
  let preview_url = move || format!("/preview/{}", &id());

  view! { cx,
    <div class="content">
      {move || {
        match &id().is_empty() {
          true => view!{cx, <h1 class="text-4xl text-center">"Invalid inscription id"</h1>}.into_view(cx),
          false => view!{cx,
            <div class="px-20 md:px-40 lg:px-60 my-10 flex items-center justify-center">
              <div class="relative w-full h-full">
                <Preview class="w-full h-full aspect-w-4 aspect-h-4 ring-4 sm:ring-8".to_owned() id=id() />
                  <a href=preview_url target=preview_url title="Click to view the unconfirmed inscription.">
                    <div class="absolute right-1 bottom-1 ease bg-white/70 hover:bg-white/90 text-gray-700 hover:text-gray-900 dark:text-gray-200 hover:dark:text-gray-50 dark:bg-slate-900/70 dark:hover:bg-slate-900/90 w-6 h-6 sm:w-10 sm:h-10 flex justify-center items-center p-1 sm:p-2 rounded-lg">
                      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" class="w-6 h-6">
                        <path strokeLinecap="round" strokeLinejoin="round" d="M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z" />
                      </svg>
                    </div>
                  </a>
              </div>
            </div>
            <Container>
              <Title>"id"</Title>
              <Label>{&id()}</Label>
            </Container>
            <Container>
              <Title>"address"</Title>
              <Label>"bc1pyqz4gyra5cssfgawk6dunh4s9car9zrzpkw2l0sz4g9ym0cdxzescnf4u2"</Label>
            </Container>
            <Container>
              <Title>"output value"</Title>
              <Label>"546"</Label>
            </Container>
            <Container>
              <Title>"sat"</Title>
              <Label>"1862722370424422"</Label>
            </Container>
            <Container>
              <Title>"content length"</Title>
              <Label>"53 bytes"</Label>
            </Container>
            <Container>
              <Title>"content type"</Title>
              <Label>"text/plain;charset=utf-8"</Label>
            </Container>
            <Container>
              <Title>"time stamp"</Title>
              <Label>"2023-04-26 13:54:04 UTC"</Label>
            </Container>
            <Container>
              <Title>"time stamp"</Title>
              <Label>"2023-04-26 13:54:04 UTC"</Label>
            </Container>
            <Container>
              <Title>"genesis height"</Title>
              <Label>"787095"</Label>
            </Container>
            <Container>
              <Title>"genesis fee"</Title>
              <Label>"2448"</Label>
            </Container>
            <Container>
              <Title>"genesis transaction"</Title>
              <Label>"a5a6f451c8f0fbd150c05b4d75c7ac45281962f3c41079d2050cc78e78243f6c"</Label>
            </Container>
            <Container>
              <Title>"location"</Title>
              <Label>"a5a6f451c8f0fbd150c05b4d75c7ac45281962f3c41079d2050cc78e78243f6c:0:0"</Label>
            </Container>
            <Container>
              <Title>"output"</Title>
              <Label>"a5a6f451c8f0fbd150c05b4d75c7ac45281962f3c41079d2050cc78e78243f6c:0"</Label>
            </Container>
            <Container>
              <Title>"offset"</Title>
              <Label>"0"</Label>
            </Container>
          }.into_view(cx),
        }
      }
    }
    <A href="/" class="flex justify-center items-center ease text-gray-400 hover:text-gray-500 dark:text-gray-300 dark:hover:text-gray-200 p-4 my-5 text-md lg:text-xl">
    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6 mr-1">
      <path stroke-linecap="round" stroke-linejoin="round" d="M11.25 9l-3 3m0 0l3 3m-3-3h7.5M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
    </svg>
    "Back"
    </A>
    </div>
  }
}

use crate::app::components::{ThemeToggle, ThemeToggleProps};
use crate::app::i18n::{Locale, T};
use leptos::*;
use leptos_router::*;
use std::str::FromStr;
use strum::IntoEnumIterator;

use crate::app::providers::*;

#[component]
pub fn Header(cx: Scope) -> impl IntoView {
  let i18n = use_context::<I18nContext>(cx).expect("Failed to get I18nContext");

  view! { cx,
    <header class="w-full">
      <nav class="bg-gray-800">
        <div class="content">
          <div class="flex h-16 justify-between">
            <div class="flex">
              <div class="-ml-2 mr-2 flex items-center md:hidden"></div>
              <div class="flex flex-shrink-0 items-center">
                <A href="/">
                  <img
                    class="block h-8 w-auto lg:hidden"
                    src="/ordilabs-logo-name-h.svg"
                    alt="Ordilabs"
                  />
                  <img
                    class="hidden h-8 w-auto lg:block"
                    src="/ordilabs-logo-name-h.svg"
                    alt="Ordilabs"
                  />
                </A>
              </div>
            </div>
            <div class="flex items-center">
              <div class="flex-shrink-0">
                <a
                  href="https://github.com/ordilabs/live"
                  target="_blank"
                  class="relative inline-flex items-center gap-x-1.5 rounded-md ease bg-red-500 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-red-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-500"
                >
                  <svg
                    class="-ml-0.5 h-5 w-5"
                    fill="currentColor"
                    viewBox="0 0 24 24"
                    aria-hidden="true"
                  >
                    <path
                      fill-rule="evenodd"
                      d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
                      clip-rule="evenodd"
                    ></path>
                  </svg>
                  {i18n.t(cx, T::ForkGH)}
                </a>
              </div>
              <ThemeToggle/>
            </div>
          </div>
        </div>
      </nav>
    </header>
  }
}

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
  let StreamContext {
    block,
    // TODO (@sectore) Remove it - just for testing serialization/deserialization LiveEvents (see #100)
    time,
    ..
  } = use_context::<StreamContext>(cx).expect("Failed to get StreamContext");

  let I18nContext {
    locale: locale_rw, ..
  } = use_context::<I18nContext>(cx).expect("Failed to get I18nContext");

  let current = move || match time.get() {
    None => String::from("--"),
    Some(v) => match v.duration_since(std::time::SystemTime::UNIX_EPOCH) {
      Err(_) => String::from("invalid"),
      Ok(v) => format!("{}", v.as_secs()),
    },
  };

  view! { cx,
    <footer class="bg-white dark:bg-slate-800">
      <div class="content flex flex-col-reverse md:flex-row md:justify-between py-2 md:py-6">
        <div class="flex justify-center md:items-center mb-2 md:mb-0 text-gray-400 dark:text-gray-400">
          <svg
            class="h-6 w-6 mr-1"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            aria-hidden="true"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"
            ></path>
          </svg>
          {block}
          " | "
          {current}
        </div>
        <div class="md:flex md:items-center my-2 md:my-0">
          <div class="flex justify-center space-x-4">
            <a
              href="https://twitter.com/OrdiLabs_org"
              class="text-gray-400 hover:text-gray-500 dark:text-gray-100 ease hover:bg-gray-200 p-2 rounded-full dark:hover:bg-white/20 "
            >
              <span class="sr-only">"Twitter"</span>
              <svg class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path d="M8.29 20.251c7.547 0 11.675-6.253 11.675-11.675 0-.178 0-.355-.012-.53A8.348 8.348 0 0022 5.92a8.19 8.19 0 01-2.357.646 4.118 4.118 0 001.804-2.27 8.224 8.224 0 01-2.605.996 4.107 4.107 0 00-6.993 3.743 11.65 11.65 0 01-8.457-4.287 4.106 4.106 0 001.27 5.477A4.072 4.072 0 012.8 9.713v.052a4.105 4.105 0 003.292 4.022 4.095 4.095 0 01-1.853.07 4.108 4.108 0 003.834 2.85A8.233 8.233 0 012 18.407a11.616 11.616 0 006.29 1.84"></path>
              </svg>
            </a>
            <a
              href="https://github.com/ordilabs/live"
              class="text-gray-400 hover:text-gray-500 dark:text-gray-100 ease hover:bg-gray-200 p-2 rounded-full dark:hover:bg-white/20"
            >
              <span class="sr-only">"GitHub"</span>
              <svg class="h-6 w-6" fill="currentColor" viewBox="0 0 24 24" aria-hidden="true">
                <path
                  fill-rule="evenodd"
                  d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"
                  clip-rule="evenodd"
                ></path>
              </svg>
            </a>
            <select
              class="focus:ring-0 border-0 background text-gray-400 bg-white dark:bg-slate-800"
              on:change=move |ev| {
                  let v = event_target_value(&ev);
                  let l = Locale::from_str(&v).unwrap();
                  locale_rw.set(l);
              }
            >
              {Locale::iter()
                  .map(|locale| {
                      let value: &'static str = locale.as_str();
                      view! { cx,
                        <option value=value selected=locale == locale_rw.get()>
                          {locale.as_str()}
                        </option>
                      }
                  })
                  .collect::<Vec<_>>()}
            </select>
          </div>
        </div>
      </div>
    </footer>
  }
}

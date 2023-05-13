use crate::app::components::{ThemeToggle, ThemeToggleProps};
use crate::app::i18n::T;
use leptos::*;
use leptos_router::*;

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

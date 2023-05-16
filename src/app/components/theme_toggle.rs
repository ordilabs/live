use crate::app::providers::ThemeContext;
use leptos::*;
use leptos_router::ActionForm;

#[component]
pub fn ThemeToggle(cx: Scope) -> impl IntoView {
  let ThemeContext {
    set_dark_action,
    is_dark,
  } = expect_context::<ThemeContext>(cx);

  view! { cx,
    <ActionForm action=set_dark_action>
      <input type="hidden" name="is_dark" value=move || (!is_dark()).to_string()/>
      <button
        type="submit"
        class="cursor-pointer rounded-full ease hover:bg-white/20 bg-transparent p-2 text-gray-100 hover:text-yellow-400 ml-2"
        inner_html=move || {
            if is_dark() {
                r#"<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true" class="w-5 h-5"><path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386l-1.591 1.591M21 12h-2.25m-.386 6.364l-1.591-1.591M12 18.75V21m-4.773-4.227l-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0z"></path></svg>"#
            } else {
                r#"<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" aria-hidden="true" class="w-5 h-5"><path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.718 9.718 0 0118 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 003 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 009.002-5.998z"></path></svg>"#
            }
        }
      ></button>
    </ActionForm>
  }
}

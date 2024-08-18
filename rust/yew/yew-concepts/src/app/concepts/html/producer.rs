use super::theme_ctx::{Theme, ThemeContext};
use yew::prelude::*;

#[function_component]
pub fn Producer() -> Html {
    let theme_ctx = use_context::<ThemeContext>().unwrap();

    let onclick = {
        let theme_ctx = theme_ctx.clone();
        Callback::from(move |_| {
            let new_theme = match *theme_ctx {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
            };
            theme_ctx.dispatch(new_theme);
        })
    };

    html! {
        <button class="hover:underline" {onclick}>
            {"toggle theme"}
        </button>
    }
}

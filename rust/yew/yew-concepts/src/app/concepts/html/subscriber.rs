use super::theme_ctx::ThemeContext;
use yew::prelude::*;

#[function_component]
pub fn Subscriber() -> Html {
    let theme_ctx = use_context::<ThemeContext>().unwrap();
    let theme = (*theme_ctx).clone();

    html! {
        <p>{format!("Current theme: {:?}", theme)}</p>
    }
}

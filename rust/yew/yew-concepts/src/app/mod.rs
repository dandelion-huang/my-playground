use yew::prelude::*;
mod concepts;
use concepts::html_fragment::HtmlFragment;

#[function_component]
pub fn App() -> Html {
    html! {
        <HtmlFragment />
    }
}

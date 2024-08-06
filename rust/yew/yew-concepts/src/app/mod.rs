use yew::prelude::*;
mod concepts;
use concepts::html_basic::HtmlBasic;

#[function_component]
pub fn App() -> Html {
    html! {
        <HtmlBasic />
    }
}

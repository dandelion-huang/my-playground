use yew::prelude::*;
mod concepts;
use concepts::html_embedded_value::HtmlEmbeddedValue;

#[function_component]
pub fn App() -> Html {
    html! {
        <HtmlEmbeddedValue />
    }
}

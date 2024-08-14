use yew::{function_component, html, Html};
mod concepts;
use concepts::html::lists::Lists;

#[function_component]
pub fn App() -> Html {
    html! {
        <Lists />
    }
}

use yew::{function_component, html, Html};
mod concepts;
use concepts::components::callbacks::Callbacks;

#[function_component]
pub fn App() -> Html {
    html! {
        <Callbacks />
    }
}

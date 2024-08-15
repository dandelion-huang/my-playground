use yew::{function_component, html, Html};
mod concepts;
use concepts::html::conditional_rendering::ConditionalRendering;

#[function_component]
pub fn App() -> Html {
    html! {
        <ConditionalRendering />
    }
}

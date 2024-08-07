use yew::{function_component, html, Html};
mod concepts;
use concepts::components::hooks::Hooks;

#[function_component]
pub fn App() -> Html {
    html! {
        <Hooks />
    }
}

use yew::{function_component, html, Html};
mod concepts;
use concepts::components::children::Children;

#[function_component]
pub fn App() -> Html {
    html! {
        <Children />
    }
}

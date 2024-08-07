use yew::{function_component, html, Html};
mod concepts;
use concepts::components::pure_components::PureComponents;

#[function_component]
pub fn App() -> Html {
    html! {
        <PureComponents />
    }
}

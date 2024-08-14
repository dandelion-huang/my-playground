use yew::{function_component, html, Html};
mod concepts;
use concepts::html::events::target_with_jscast::TargetWithJsCast;

#[function_component]
pub fn App() -> Html {
    html! {
        <TargetWithJsCast />
    }
}

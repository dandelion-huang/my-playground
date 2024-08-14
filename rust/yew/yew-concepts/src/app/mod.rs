use yew::{function_component, html, Html};
mod concepts;
use concepts::html::events::target_with_targetcast::TargetWithTargetCast;

#[function_component]
pub fn App() -> Html {
    html! {
        <TargetWithTargetCast />
    }
}

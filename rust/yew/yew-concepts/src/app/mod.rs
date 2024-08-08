use yew::{function_component, html, Html};
mod concepts;
use concepts::components::node_ref_scroll_to_li::NodeRefScrollToLi;

#[function_component]
pub fn App() -> Html {
    html! {
        <NodeRefScrollToLi />
    }
}

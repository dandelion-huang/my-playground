use yew::{function_component, html, Html};
mod concepts;
use concepts::html::events::callback_with_node_ref::CallbackWithNodeRef;

#[function_component]
pub fn App() -> Html {
    html! {
        <CallbackWithNodeRef />
    }
}

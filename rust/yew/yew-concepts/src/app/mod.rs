use yew::{function_component, html, Html};
mod concepts;
use concepts::html::events::manual_event_listener_with_closure::ManualEventListenerWithClosure;

#[function_component]
pub fn App() -> Html {
    html! {
        <ManualEventListenerWithClosure />
    }
}

use yew::{function_component, html, Html};
mod concepts;
use concepts::components::callbacks_and_dom_events::CallbacksAndDomEvents;

#[function_component]
pub fn App() -> Html {
    html! {
        <CallbacksAndDomEvents />
    }
}

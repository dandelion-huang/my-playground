use yew::{function_component, html, Html};
mod concepts;
use concepts::components::callbacks_as_props::CallbacksAsProps;

#[function_component]
pub fn App() -> Html {
    html! {
        <CallbacksAsProps />
    }
}

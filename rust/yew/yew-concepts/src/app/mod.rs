use yew::{function_component, html, Html};
mod concepts;
use concepts::components::persistent_inputs::PersistentInputs;

#[function_component]
pub fn App() -> Html {
    html! {
        <PersistentInputs />
    }
}

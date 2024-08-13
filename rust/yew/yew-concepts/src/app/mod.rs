use yew::{function_component, html, Html};
mod concepts;
use concepts::components::generic_component::MyGenericComponent;

#[function_component]
pub fn App() -> Html {
    html! {
        <MyGenericComponent />
    }
}

use yew::{function_component, html, Html};
mod concepts;
use concepts::components::properties_prop_macro::PropertiesPropMacro;

#[function_component]
pub fn App() -> Html {
    html! {
        <PropertiesPropMacro />
    }
}

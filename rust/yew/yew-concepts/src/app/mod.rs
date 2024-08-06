use yew::{function_component, html, Html};
mod concepts;
use concepts::components::properties_prop_or_else_function::PropertiesPropOrElseFunction;

#[function_component]
pub fn App() -> Html {
    html! {
        <PropertiesPropOrElseFunction />
    }
}

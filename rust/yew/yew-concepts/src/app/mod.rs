use yew::{function_component, html, Html};
mod concepts;
use concepts::components::properties_prop_or_value::PropertiesPropOrValue;

#[function_component]
pub fn App() -> Html {
    html! {
        <PropertiesPropOrValue />
    }
}

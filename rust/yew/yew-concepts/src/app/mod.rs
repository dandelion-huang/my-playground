use yew::{function_component, html, Html};
mod concepts;
use concepts::components::properties_props::PropertiesProps;

#[function_component]
pub fn App() -> Html {
    html! {
        <PropertiesProps />
    }
}

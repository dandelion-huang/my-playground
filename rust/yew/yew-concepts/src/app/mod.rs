use yew::{function_component, html, Html};
mod concepts;
use concepts::html::classes_macro::ClassesMacro;

#[function_component]
pub fn App() -> Html {
    html! {
        <ClassesMacro />
    }
}

use yew::{function_component, html, Html};
mod concepts;
use concepts::html::literals_and_expressions::LiteralsAndExpressions;

#[function_component]
pub fn App() -> Html {
    html! {
        <LiteralsAndExpressions />
    }
}

use yew::prelude::*;
mod concepts;
use concepts::html::router::Router;

#[function_component]
pub fn App() -> Html {
    html! {
        <Router />
    }
}

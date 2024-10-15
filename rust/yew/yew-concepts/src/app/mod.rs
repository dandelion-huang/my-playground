use yew::prelude::*;
mod concepts;
use concepts::contexts::Contexts;

#[function_component]
pub fn App() -> Html {
    html! {
        <Contexts />
    }
}

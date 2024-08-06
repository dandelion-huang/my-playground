use yew::prelude::*;
mod concepts;
use concepts::html_img::HtmlImg;

#[function_component]
pub fn App() -> Html {
    html! {
        <HtmlImg />
    }
}

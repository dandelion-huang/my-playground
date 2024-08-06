use yew::prelude::*;

#[function_component]
pub fn HtmlFragment() -> Html {
    html! {
        <>
            <h1 align="center">{"Hello, Rust!"}</h1>
            <h2 align="center">{"Hello, Rust!"}</h2>
            <h3 align="center">{"Hello, Rust!"}</h3>
            <h4 align="center">{"Hello, Rust!"}</h4>
            <h5 align="center">{"Hello, Rust!"}</h5>
            <h6 align="center">{"Hello, Rust!"}</h6>
        </>
    }
}

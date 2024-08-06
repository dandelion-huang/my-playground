use yew::prelude::*;

#[function_component]
pub fn HtmlEmbeddedValue() -> Html {
    let header_text = "Hello, world!";
    let header_html = html! {
        <h1>{header_text}</h1>
    };

    let count: usize = 31;
    let counter_html = html! {
        <p>{"My age is: "}{count}</p>
    };

    let combined_html = html! {
        <>
            {header_html}
            {counter_html}
        </>
    };

    combined_html
}

use yew::prelude::*;

// If it implemented trait Display, it would be rendered as text.
// Text is an expression.

#[function_component]
fn Literals() -> Html {
    let text: &str = "Lorem ipsum";

    html! {
        <>
            <div>{text}</div>
            <div>{"dolot sit"}</div>
            <div>{42}</div>
        </>
    }
}

#[function_component]
fn Expressions() -> Html {
    let should_show: bool = true;
    let maybe_display_link = move || -> Html {
        if should_show {
            html! {
                <a class="text-blue-500 hover:underline" href="https://www.rust-lang.org/">{"Rust official website"}</a>
            }
        } else {
            html! {}
        }
    };

    html! {
        <>
            {maybe_display_link()}
        </>
    }
}

#[function_component]
pub fn LiteralsAndExpressions() -> Html {
    html! {
        <div class="flex flex-col gap-2 items-center justify-center w-full h-[100dvh] bg-gray-100">
            <h1 class="text-2xl">{"Literals and expressions"}</h1>
            <Literals />
            <Expressions />
        </div>
    }
}

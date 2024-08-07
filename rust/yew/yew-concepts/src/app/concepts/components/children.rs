use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component]
fn ParentComponent(props: &Props) -> Html {
    html! {
        <div class="flex items-center justify-center w-full h-screen bg-gray-100">
        <div class="p-10 bg-green-100">//
            <h1 class="text-2xl p-2 text-center">{"Children"}</h1>
                <div class="flex items-center justify-center gap-2">
                    {props.children.clone()}
                </div>
            </div>
        </div>
    }
}

#[function_component]
fn Child() -> Html {
    html! {
        <div class="p-10 bg-red-100">
            <p>{"Hi, I'm a child."}</p>
        </div>
    }
}

#[function_component]
pub fn Children() -> Html {
    html! {
        <ParentComponent>
            <Child />
            <Child />
            <Child />
        </ParentComponent>
    }
}

// A function component is considered pure when the returned Html is deterministically derived from its props when its view function does not mutate its state or has other side effects.

use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub is_loading: bool,
}

#[function_component]
fn PureComponent(props: &Props) -> Html {
    if props.is_loading {
        html! { "Loading..." }
    } else {
        html! { "Hello, Rust!" }
    }
}

#[function_component]
pub fn PureComponents() -> Html {
    html! {
        <div class="flex items-center justify-center w-full h-[100dvh] bg-gray-100">
            <PureComponent is_loading={true} />
            <PureComponent />
        </div>
    }
}

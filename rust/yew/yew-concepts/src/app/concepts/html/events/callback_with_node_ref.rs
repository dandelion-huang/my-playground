use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
fn MyComponent() -> Html {
    let input_node_ref: NodeRef = use_node_ref();
    let input_value_handle: UseStateHandle<String> = use_state(String::default);
    let input_value: String = (*input_value_handle).clone();
    let onchange: Callback<_> = {
        let input_node_ref: NodeRef = input_node_ref.clone();
        Callback::from(move |_: Event| {
            // Notice that we ingone the event here.
            // From node_ref in Rust cast to Html Element.
            let input: Option<HtmlInputElement> = input_node_ref.cast::<HtmlInputElement>();
            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };

    html! {
        <>
            <label for="my-input">
                {"My input: "}
                <input
                    id="my-input"
                    type="text"
                    value={input_value}
                    {onchange}
                />
            </label>
        </>
    }
}

#[function_component]
pub fn CallbackWithNodeRef() -> Html {
    html! {
        <div class="flex flex-col gap-2 items-center justify-center w-full h-[100dvh] bg-gray-100">
            <MyComponent />
        </div>
    }
}

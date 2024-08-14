use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

#[function_component]
fn MyComponent() -> Html {
    let input_value_handle: UseStateHandle<String> = use_state(String::default);
    let input_value: String = (*input_value_handle).clone();

    let on_cautious_change: Callback<Event> = {
        let input_value_handle: UseStateHandle<String> = input_value_handle.clone();
        Callback::from(move |event: Event| {
            // When events are created the target is undefined, it's only
            // when dispatched does the target get added.
            let target: Option<EventTarget> = event.target();
            // Events can bubble so this listener might catch events from child
            // elements which are not of type HtmlInputElement.
            // * Yew only supports event bubbling, no capturing.
            //   and the event target is pointing to the ref in Rust.
            //   The event will alwayes be triggered in Rust and then casted to Js.
            let input: Option<HtmlInputElement> =
                target.and_then(|t: EventTarget| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };

    let on_dangerous_change: Callback<Event> = Callback::from(move |event: Event| {
        let target: EventTarget = event
            .target()
            .expect("Event should have a target when dispatched");
        // You must KNOW target is a HtmlInputElement, otherwise
        // the call to value would be Undefined Behaviour (UB).
        // Here we are sure that this is input element so we can convert it
        // to the appropriate type without checking
        input_value_handle.set(target.unchecked_into::<HtmlInputElement>().value());
    });

    html! {
        <>
            <label for="cautious-input">
                {"Cautious input: "}
                <input
                    id="cautious-input"
                    type="text"
                    value={input_value.clone()}
                    onchange={on_dangerous_change}
                />
            </label>
            <label for="dangerous-input">
                {"Dangerous input: "}
                <input
                    id="dangerous-input"
                    type="text"
                    value={input_value}
                    onchange={on_cautious_change}
                />
            </label>
        </>
    }
}

#[function_component]
pub fn TargetWithJsCast() -> Html {
    html! {
        <div class="flex flex-col gap-2 items-center justify-center w-full h-[100dvh] bg-gray-100">
            <MyComponent />
        </div>
    }
}

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
fn MyComponent() -> Html {
    let input_value_handle: UseStateHandle<String> = use_state(String::default);
    let input_value: String = (*input_value_handle).clone();

    let on_cautious_change: Callback<Event> = {
        let input_value_handle: UseStateHandle<String> = input_value_handle.clone();
        // TargetCast is designed to be very similar to JsCast
        // but it's with the smaller scope of the event target.
        Callback::from(move |event: Event| {
            let input = event.target_dyn_into::<HtmlInputElement>();
            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };

    let on_dangerous_change: Callback<Event> = Callback::from(move |event: Event| {
        // You must KNOW target is a HtmlInputElement, otherwise
        // the call to value would be Undefined Behaviour (UB).
        let target: String = event.target_unchecked_into::<HtmlInputElement>().value();
        input_value_handle.set(target);
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
pub fn TargetWithTargetCast() -> Html {
    html! {
        <div class="flex flex-col gap-2 items-center justify-center w-full h-[100dvh] bg-gray-100">
            <MyComponent />
        </div>
    }
}

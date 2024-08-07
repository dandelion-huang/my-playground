// src/app/concepts/components/callbacks_and_dom_events.rs
// revise use_state to use_state_eq

use super::callbacks_and_dom_events::ChildComponent;
use web_sys::{console, js_sys::Date};
use yew::{
    classes, function_component, html, use_state_eq, Callback, Html, MouseEvent, UseStateHandle,
};

#[function_component]
fn ParentComponent() -> Html {
    let counter: UseStateHandle<u64> = use_state_eq(|| 0u64); // 使用 use_state_eq
    let update_counter: Callback<_> = {
        let counter: UseStateHandle<u64> = counter.clone();
        Callback::from(move |_: MouseEvent| counter.set(*counter + 1))
    };
    let keep_counter: Callback<_> = {
        let counter: UseStateHandle<u64> = counter.clone();
        Callback::from(move |_: MouseEvent| counter.set(*counter))
    };

    let bg_class = match *counter % 2 == 0 {
        true => "bg-red-100",
        false => "bg-green-100",
    };

    let timestamp = Date::now();
    console::log_1(&format!("Counter: {}, Timestamp: {}", *counter, timestamp).into());

    html! {
        <div class="flex items-center justify-center w-full h-screen bg-gray-100">
            <div class={classes!("p-10", "space-y-2", "rounded-md", bg_class)}>
                <h1 class="text-2xl">{"Callbacks and DOM Events"}</h1>
                <p>{"Click the button below to increment the counter."}</p>
                <p>{format!("Counter: {}", *counter)}</p>
                <p>
                    {"If counter is odd, the background-color will be"}
                    {" "}
                    <span class="text-red-500">{"red"}</span>
                    {"."}
                </p>
                <p>
                    {"If counter is odd, the background-color will be"}
                    {" "}
                    <span class="text-green-500">{"green"}</span>
                    {"."}
                </p>
                <div class="!mt-4 space-x-2">
                    <ChildComponent {update_counter}>
                        {"Re-render"}
                    </ChildComponent>
                    <ChildComponent update_counter={keep_counter}>
                        {"Not re-render"}
                    </ChildComponent>
                </div>
            </div>
        </div>
    }
}

#[function_component]
fn CallbacksAndDomEventsWithUseStateEq() -> Html {
    html! {
        <ParentComponent />
    }
}

#[function_component]
pub fn Hooks() -> Html {
    html! {
        <CallbacksAndDomEventsWithUseStateEq />
    }
}
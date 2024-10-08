use web_sys::{console, js_sys::Date};
use yew::{
    classes, function_component, html, use_state, Callback, Html, MouseEvent, Properties,
    UseStateHandle,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    pub update_counter: Callback<MouseEvent>,
}

#[function_component]
pub fn ChildComponent(props: &Props) -> Html {
    let onclick: Callback<_> = props.update_counter.clone();

    html! {
        <button class="bg-blue-500 px-4 py-2 rounded-md text-white hover:bg-blue-400 transtion-colors duration-200" {onclick}>
            {props.children.clone()}
        </button>
    }
}

#[function_component]
fn ParentComponent() -> Html {
    let counter: UseStateHandle<u64> = use_state(|| 0u64); // use_state always re-render
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
            <div class={classes!("p-10", "space-y-2", bg_class)}>
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
                        {"Click me!"}
                    </ChildComponent>
                    <ChildComponent update_counter={keep_counter}>
                        {"Re-render!?"}
                    </ChildComponent>
                </div>
            </div>
        </div>
    }
}

#[function_component]
pub fn CallbacksAndDomEvents() -> Html {
    html! {
        <ParentComponent />
    }
}

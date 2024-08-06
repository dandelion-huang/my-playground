use yew::{
    classes, function_component, html, use_state, Callback, Html, MouseEvent, Properties,
    UseStateHandle,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub update_counter: Callback<MouseEvent>,
}

#[function_component]
fn ChildComponent(props: &Props) -> Html {
    let onclick: Callback<_> = props.update_counter.clone();

    html! {
        <button class="bg-blue-500 px-4 py-2 rounded-md text-white hover:bg-blue-400 transtion-colors duration-200" {onclick}>{ "Click me!" }</button>
    }
}

#[function_component]
fn ParentComponent() -> Html {
    let counter: UseStateHandle<u64> = use_state(|| 0u64);
    let update_counter: Callback<_> = {
        let counter: UseStateHandle<u64> = counter.clone();
        Callback::from(move |_: MouseEvent| counter.set(*counter + 1))
    };

    let bg_class = match *counter % 2 == 0 {
        true => "bg-red-100",
        false => "bg-green-100",
    };

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
                <ChildComponent {update_counter} />
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

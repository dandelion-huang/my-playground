use yew::prelude::*;
use web_sys::window;

#[function_component]
pub fn WebSysAlert() -> Html {
    let window = window().unwrap();
    let _ = window.alert_with_message("Hello, world!");

    html! {
        <div>
            <h1 class="text-8xl">{"web_sys, alert!"}</h1>
        </div>
    }
}

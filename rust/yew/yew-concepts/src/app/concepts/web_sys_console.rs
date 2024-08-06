// https://rustwasm.github.io/docs/wasm-bindgen/examples/console-log.html

use yew::prelude::*;
use web_sys::console;
use wasm_bindgen::JsValue;

#[function_component]
pub fn WebSysConsole() -> Html {
    console::log_1(&"Hello using web-sys".into());

    let js: JsValue = 4.into();
    console::log_2(&"Logging arbitrary values looks like".into(), &js);

    html! {
        <div>
            <h1 class="text-8xl">{"Please check the console"}</h1>
        </div>
    }
}

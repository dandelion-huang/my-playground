use yew::prelude::*;
mod concepts;
use concepts::wasm_bindgen_console::WasmBindgenConsole;

#[function_component]
pub fn App() -> Html {
    html! {
        <WasmBindgenConsole />
    }
}

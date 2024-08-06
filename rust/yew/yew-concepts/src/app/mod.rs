use yew::prelude::*;
mod concepts;
use concepts::web_sys_console::WebSysConsole;

#[function_component]
pub fn App() -> Html {
    html! {
        <WebSysConsole />
    }
}

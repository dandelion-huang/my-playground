use yew::prelude::*;
mod concepts;
use concepts::web_sys_mousemove::WebSysMouseMove;

#[function_component]
pub fn App() -> Html {
    html! {
        <WebSysMouseMove />
    }
}

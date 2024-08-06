use yew::prelude::*;
mod concepts;
use concepts::web_sys_alert::WebSysAlert;

#[function_component]
pub fn App() -> Html {
    html! {
        <WebSysAlert />
    }
}

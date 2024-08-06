use web_sys::{console, HtmlElement, MouseEvent};
use yew::prelude::*;

#[function_component]
pub fn WebSysMouseMove() -> Html {
    let onmousemove = Callback::from(|e: MouseEvent| {
        if let Some(target) = e.target_dyn_into::<HtmlElement>() {
            let rect = target.get_bounding_client_rect();
            let x = (e.client_x() as f64) - rect.left();
            let y = (e.client_y() as f64) - rect.top();
            console::log_1(&format!("Left? : {} ; Top? : {}", x, y).into());
        }
    });

    html! {
        <div class="flex justify-center items-center w-full h-[100dvh] bg-yellow-200">
            <div class="w-80 h-80 bg-blue-200" {onmousemove}/>
        </div>
    }
}

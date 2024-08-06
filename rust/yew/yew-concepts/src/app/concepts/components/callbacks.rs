use web_sys::console;
use yew::{function_component, html, Callback, Html};

#[function_component]
pub fn Callbacks() -> Html {
    let cb: Callback<String, String> = Callback::from(move |name: String| format!("Bye {}", name));

    let result: String = cb.emit(String::from("Bob")); // call the callback
    console::log_1(&result.into()); // if uncommented will print "Bye Bob"

    html! {
        <div>
            <h1 class="text-2xl">{"Callbacks"}</h1>
        </div>
    }
}

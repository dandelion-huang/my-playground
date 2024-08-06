use web_sys::console;
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_name_entry: Callback<String>,
}

#[function_component]
fn HelloWorld(props: &Props) -> Html {
    props.on_name_entry.emit(String::from("Bob"));

    html! {
        "Callbacks as props"
    }
}

#[function_component]
pub fn CallbacksAsProps() -> Html {
    let on_name_entry: Callback<String> = Callback::from(move |name: String| {
        let greeting: String = format!("Hey, {}!", name);
        console::log_1(&greeting.into()); // if uncommented will print
    });

    html! { <HelloWorld {on_name_entry} /> }
}

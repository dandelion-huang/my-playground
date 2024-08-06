use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or("Bob".to_string())]
    pub name: String,
}

#[function_component]
fn Greet(props: &Props) -> Html {
    html! {
        <div>
            {"My name is? - "}{props.name.clone()}
        </div>
    }
}

#[function_component]
pub fn PropertiesPropOrValue() -> Html {
    html! {
        <>
            <h1 class="text-2xl">{"Properties #[prop_or(value)]"}</h1>
            <Greet name={"Dandelion"} />
            <Greet />
        </>
    }
}

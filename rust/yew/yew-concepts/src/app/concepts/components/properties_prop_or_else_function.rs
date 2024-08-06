use yew::{function_component, html, Html, Properties};

fn create_default_name() -> String {
    "Ginny".to_string()
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_else(create_default_name)]
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
pub fn PropertiesPropOrElseFunction() -> Html {
    html! {
        <>
            <h1 class="text-2xl">{"Properties #[prop_or_else(function)]"}</h1>
            <Greet name={"Abbie"} />
            <Greet />
        </>
    }
}

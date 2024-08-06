use yew::{function_component, html, props, virtual_dom::AttrValue, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(AttrValue::from("Eddie"))]
    pub name: AttrValue, // AttrValue is preferred over String
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
pub fn PropertiesPropMacro() -> Html {
    let pre_made_props = props! {
        Props {}
    };

    html! {
        <>
            <h1 class="text-2xl">{"Properties & props! Macro"}</h1>
            <Greet name={"Bob"} />
            <Greet ..pre_made_props />
        </>
    }
}

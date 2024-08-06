use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub is_loading: bool, // default is false
}

#[function_component]
fn IsLoading(props: &Props) -> Html {
    html! {
        <div>
            {"Am I loading? - "}{props.is_loading.clone()}
        </div>
    }
}

#[function_component]
pub fn PropertiesPropOrDefault() -> Html {
    html! {
        <>
            <h1 class="text-2xl">{"Properties #[prop_or_default]"}</h1>
            <IsLoading is_loading={true} />
            <IsLoading />
        </>
    }
}

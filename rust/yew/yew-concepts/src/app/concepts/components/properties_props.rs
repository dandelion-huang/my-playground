use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_loading: bool,
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
pub fn PropertiesProps() -> Html {
    html! {
        <>
            <h1 class="text-2xl">{"Properties"}</h1>
            <IsLoading is_loading={true} />
            <IsLoading is_loading={false} />
        </>
    }
}

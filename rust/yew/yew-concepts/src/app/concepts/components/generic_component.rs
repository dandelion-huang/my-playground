use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props<T>
where
    T: PartialEq,
{
    data: T,
}

#[function_component]
pub fn GenericComponent<T>(props: &Props<T>) -> Html
where
    T: PartialEq + ToHtml,
{
    html! {
        <p>
            { &props.data }
        </p>
    }
}

#[function_component]
pub fn MyGenericComponent() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center w-full h-[100dvh] bg-gray-100">
            <GenericComponent<String> data={"Hello, Rust!".to_string()} />
            <GenericComponent<&'static str> data="Rust?" />
            <GenericComponent<char> data='R' />
            <GenericComponent<u32> data=777 />
            <GenericComponent<f32> data=6.6 />
            <GenericComponent<bool> data=true />
            <GenericComponent<Vec<u8>> data={vec![1, 3, 5]} />
            <GenericComponent<Option<u8>> data={Some(8)} />
        </div>
    }
}

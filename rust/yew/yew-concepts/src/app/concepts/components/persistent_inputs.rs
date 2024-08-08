use super::use_local_storage::use_local_storage;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub storage_key: AttrValue,
}

#[function_component]
fn PersistentInput(props: &Props) -> Html {
    let (value, set_value) = use_local_storage(&props.storage_key, String::new());

    let oninput: Callback<InputEvent> = Callback::from(move |e: InputEvent| {
        let input: HtmlInputElement = e.target_unchecked_into();
        set_value.emit(input.value());
    });

    html! {
        <input
            type="text"
            value={value}
            {oninput}
            placeholder="Type something..."
        />
    }
}

#[function_component]
pub fn PersistentInputs() -> Html {
    html! {
        <div class="flex items-center justify-center w-full h-[100dvh] bg-gray-100">
            <div class="flex flex-col gap-4 p-10 space-y-2 rounded-md bg-blue-100">
                <PersistentInput storage_key="input-content-1" />
                <PersistentInput storage_key="input-content-2" />
            </div>
        </div>
    }
}

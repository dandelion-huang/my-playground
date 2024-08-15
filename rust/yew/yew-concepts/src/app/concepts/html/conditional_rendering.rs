use yew::prelude::*;

#[function_component]
fn If() -> Html {
    html! {
        if true {
            <div>{"If: true case"}</div>
        }
    }
}

#[function_component]
fn IfElse() -> Html {
    html! {
        if false {
            <div>{"If else: true case"}</div>
        } else {
            <div>{"If else: false case"}</div>
        }
    }
}

#[function_component]
fn IfLet() -> Html {
    let some_value: Option<i32> = Some(42);

    html! {
        if let Some(value) = some_value {
            <div>{format!("If let: Some({})", value)}</div>
        }
    }
}

#[function_component]
fn IfLetElse() -> Html {
    let some_value: Option<i32> = None;

    html! {
        if let Some(value) = some_value {
            <div>{format!("If let else: Some({})", value)}</div>
        } else {
            <div>{"If let else: None case"}</div>
        }
    }
}

#[function_component]
pub fn ConditionalRendering() -> Html {
    html! {
        <div class="flex flex-col gap-2 items-center justify-center w-full h-[100dvh] bg-gray-100">
            <h1 class="text-2xl">{"Conditional rendering"}</h1>
            <If />
            <IfElse />
            <IfLet />
            <IfLetElse />
        </div>
    }
}

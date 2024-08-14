use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub fill: bool,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
fn MyComponent(props: &Props) -> Html {
    let Props {
        class,
        fill,
        children,
    } = props;

    html! {
        <div
            class={classes!(
                "bg-blue-100",
                fill.then(|| Some("bg-red-100")),
                class.clone(),
            )}
        >
            { children.clone() }
        </div>
    }
}

#[function_component]
pub fn ClassesMacro() -> Html {
    let classes = "px-8 py-2 rounded-md text-center";
    let hover_classes =
        "transition-colors hover:text-white hover:bg-black duration-200 cursor-pointer";

    html! {
        <div class="flex flex-col gap-2 items-center justify-center w-full h-[100dvh] bg-gray-100">
            <MyComponent class={classes!(classes, "space-y-2", "bg-green-100")}>
                <MyComponent class={classes!(classes, hover_classes)} fill=true>
                    {"AHAHA"}
                </MyComponent>
                <MyComponent class={classes!(classes, hover_classes)}>
                    {"BUWAH"}
                </MyComponent>
                <MyComponent class={classes!(classes, hover_classes)} fill=true>
                    {"CUCUCU"}
                </MyComponent>
            </MyComponent>
        </div>
    }
}

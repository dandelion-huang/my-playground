use super::theme_ctx::{Theme, ThemeContext};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ThemeWrapperProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn ThemeWrapper(props: &ThemeWrapperProps) -> Html {
    let theme_ctx = use_context::<ThemeContext>().unwrap();
    let theme = (*theme_ctx).clone();
    let theme_background_class = match theme {
        Theme::Light => "bg-gray-100",
        Theme::Dark => "bg-black",
    };
    let theme_foreground_class = match theme {
        Theme::Light => "text-black",
        Theme::Dark => "text-white",
    };
    let classes = "flex flex-col gap-2 items-center justify-center w-full h-[100dvh] transition-colors duration-500";

    html! {
        <div class={classes!(classes, theme_background_class, theme_foreground_class)}>
            <h1 class="text-2xl">{"Theme Context"}</h1>
            {props.children.clone()}
        </div>
    }
}

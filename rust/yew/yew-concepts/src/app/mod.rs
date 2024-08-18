use yew::prelude::*;
mod concepts;
use concepts::html::{
    producer::Producer, subscriber::Subscriber, theme_ctx::ThemeProvider,
    theme_wrapper::ThemeWrapper,
};

#[function_component]
pub fn App() -> Html {
    html! {
        <ThemeProvider>
            <ThemeWrapper>
                <Producer />
                <Subscriber />
            </ThemeWrapper>
        </ThemeProvider>
    }
}

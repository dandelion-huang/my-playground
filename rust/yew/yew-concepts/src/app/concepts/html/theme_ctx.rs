use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Reducible for Theme {
    type Action = Theme;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        action.into()
    }
}

pub type ThemeContext = UseReducerHandle<Theme>;

#[derive(Debug, PartialEq, Properties)]
pub struct ThemeProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn ThemeProvider(props: &ThemeProviderProps) -> Html {
    let theme = use_reducer(|| Theme::Light);

    html! {
        <ContextProvider<ThemeContext> context={theme}>
            {props.children.clone()}
        </ContextProvider<ThemeContext>>
    }
}

use yew::prelude::*;
mod concepts;
use concepts::class_and_classes_macro::ClassAndClassesMacro;

#[function_component]
pub fn App() -> Html {
    html! {
        <ClassAndClassesMacro />
    }
}

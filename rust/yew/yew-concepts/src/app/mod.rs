use yew::prelude::*;
mod concepts;
use concepts::contexts::context_theme::Component;

#[function_component]
pub fn App() -> Html {
	html! {
		<Component />
	}
}

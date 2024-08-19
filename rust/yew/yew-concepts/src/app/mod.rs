use yew::prelude::*;
mod concepts;
use concepts::html::suspense_github_query::SuspenseGithubQuery;

#[function_component]
pub fn App() -> Html {
    html! {
        <SuspenseGithubQuery />
    }
}

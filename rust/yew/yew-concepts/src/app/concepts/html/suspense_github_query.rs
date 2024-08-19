use async_std;
use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum FetchStatus {
    Idle,
    Loading,
    Success,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct RepoData {
    pub full_name: String,
    pub html_url: String,
}

#[derive(Clone, PartialEq)]
pub struct GithubState {
    data: Vec<RepoData>,
    status: FetchStatus,
}

#[function_component]
pub fn SuspenseGithubQuery() -> Html {
    let state = use_state(|| GithubState {
        data: vec![],
        status: FetchStatus::Idle,
    });

    let get_data = {
        let state = state.clone();

        Callback::from(move |query: String| {
            let state = state.clone();
            async_std::task::spawn_local(async move {
                state.set(GithubState {
                    data: state.data.clone(),
                    status: FetchStatus::Loading,
                });

                let url = format!("https://api.github.com/search/repositories?q={}", query);
                // task_sender.send()
                let resp = Request::get(&url).send().await.unwrap();
                let result: serde_json::Value = resp.json().await.unwrap();
                let items = result["items"].as_array().unwrap();
                let data: Vec<RepoData> = serde_json::from_value(items.clone().into()).unwrap();

                state.set(GithubState {
                    data,
                    status: FetchStatus::Success,
                });
            });
        })
    };

    html! {
        <div class="flex flex-col gap-2 items-center justify-center w-full min-h-[100dvh] p-8 bg-gray-100">
            <h1 class="text-2xl">{"Suspense"}</h1>
            <button class="bg-blue-500 px-4 py-2 rounded-md text-white hover:bg-blue-400 transition-colors duration-200" onclick={Callback::from(move |_| get_data.emit("rust".to_string()))}>
                {"Search Rust Repos"}
            </button>
            <p>{"Status: "}{format!("{:?}", state.status)}</p>
            <ul class="shrink-0">
                {state.data.iter().map(|repo| {
                    html! {
                        <li class="text-center">
                            <a class="hover:underline hover:text-blue-500" href={repo.html_url.clone()} target="_blank">{&repo.full_name}</a>
                        </li>
                    }
                }).collect::<Html>()}
            </ul>
        </div>
    }
}

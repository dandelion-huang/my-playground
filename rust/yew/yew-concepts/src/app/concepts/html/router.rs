use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/post/:id")]
    Post { id: u8 },
    #[not_found]
    #[at("/not-found")]
    NotFound,
}

#[function_component]
fn Home() -> Html {
    html! {
        <h1 class="text-blue-500">{"Home"}</h1>
    }
}

#[derive(Properties, PartialEq)]
struct PostProps {
    id: u8,
}

#[function_component]
fn Post(props: &PostProps) -> Html {
    html! {
        <>
            <h1 class="text-green-500">{"Post:"} {&props.id}</h1>
            <p>{"If id is 0, redirect to home page."}</p>
        </>
    }
}

#[function_component]
fn NotFound() -> Html {
    html! {
        <h1 class="text-red-500">{"404 - Page Not Found"}</h1>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Post { id } => match id {
            0 => html! { <Redirect<Route> to={Route::Home}/>},
            _ => html! { <Post {id} /> },
        },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component]
fn Navbar() -> Html {
    let navigator = use_navigator().unwrap();

    let goto = {
        let navigator = navigator.clone();
        Callback::from(move |route: Route| navigator.push(&route))
    };

    let class = "shrink-0 px-4 py-2 rounded-md transition-colors hover:text-white hover:bg-black duration-200";
    let home_class = "bg-blue-700 text-blue-200";
    let post_class = "bg-green-700 text-green-200";
    let not_found_class = "bg-red-700 text-red-200";

    html! {
        <nav class="fixed top-4 left-1/2 -translate-x-1/2 flex flex-col sm:flex-row gap-4 w-max p-4 bg-gray-200 rounded-md">
            <Link<Route> classes={classes!(class, home_class)} to={Route::Home}>{"Home"}</Link<Route>>
            <button class={classes!(class, post_class)} onclick={goto.reform(|_| Route::Post { id: 255 })}>{"Post"}</button>
            <button class={classes!(class, not_found_class)} onclick={goto.reform(|_| Route::NotFound)}>{"Not Found"}</button>
        </nav>
    }
}

#[function_component]
pub fn Router() -> Html {
    html! {
        <BrowserRouter>
            <div class="flex flex-col gap-2 items-center justify-center w-full h-[100dvh] bg-gray-100">
                <Navbar />
                <Switch<Route> render={switch} />
            </div>
        </BrowserRouter>
    }
}

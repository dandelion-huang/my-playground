use yew::prelude::*;

const NAMES: &[&str] = &["Abby", "Brenda", "Carla"];

fn name_to_html(name: &String) -> Html {
    html! {
        <div key={name.clone()}>
            {format!("Hi, I'm {}!", name)}
        </div>
    }
}

#[function_component]
pub fn Lists() -> Html {
    let mut names: Vec<String> = NAMES.iter().map(|&s| s.to_string()).collect();
    if let Some(last) = names.last_mut() {
        *last = "Christine".to_string();
    }

    html! {
        <div class="flex flex-col gap-2 items-center justify-center w-full h-[100dvh] bg-gray-100">
            <h2 class="text-2xl">{"iter().map(fn).collect()"}</h2>
            {
                names.iter().map(name_to_html).collect::<Html>()
            }
            <br />
            <h2 class="text-2xl">{"for iter().map(fn)"}</h2>
            {
                for names.iter().map(name_to_html)
            }
        </div>
    }
}

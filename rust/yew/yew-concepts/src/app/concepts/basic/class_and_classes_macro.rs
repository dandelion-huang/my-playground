use yew::prelude::*;
use rand::Rng;

#[function_component]
pub fn ClassAndClassesMacro() -> Html {
    let header_text = "Hello, world!";
    let header_html = html! {
        <h1 class="text-green-500 text-3xl">{header_text}</h1>
    };

    let count: usize = 31;
    let counter_html = html! {
        <p class={classes!("text-blue-500", "text-8xl")}>{"My age is: "}{count}</p>
    };

    let string_class = String::from("cursor-pointer hover:text-red-500 text-2xl");
    let string_class_html = html! {
        <p class={classes!(string_class)}>{"This is a string class"}</p>
    };

    let mut rng = rand::thread_rng();
    let rand_integer = rng.gen_range(0..10);
    let rand_float = rng.gen_range(0.0..1.0);
    let formatted_rand_float = format!("{:.1}", rand_float);
    let formatted_float_value: f64 = formatted_rand_float.parse().unwrap();
    let optional_class = match formatted_float_value > 0.5 {
        true => Some("text-2xl text-red-500"),
        false => None,
    };
    let rand_html = html! {
        <>
            <div>{rand_integer}</div>
            <div>{rand_float}</div>
            <div class={classes!(optional_class)}>{formatted_rand_float}</div>
        </>
    };

    let vec_html = html! {
        <div class={classes!(vec!["text-yellow-700", "border-2"])}>{"vector classes!"}</div>
    };

    let slice_html = html! {
        <div class={classes!(["text-cyan-700", "border-2"].as_ref())}>{"slice classes!"}</div>
    };

    let inline_html = html! {
        <div style="color: purple;">{"inline html!"}</div>
    };

    let combined_html = html! {
        <div class="w-3/4 mx-auto p-4 flex flex-col gap-4">
            {header_html}
            {counter_html}
            {string_class_html}
            {rand_html}
            {vec_html}
            {slice_html}
            {inline_html}
        </div>
    };

    combined_html
}

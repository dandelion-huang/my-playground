use wasm_bindgen::JsCast;
use web_sys::{console, Element, HtmlElement, ScrollBehavior, ScrollIntoViewOptions};
use yew::prelude::*;

#[function_component]
pub fn NodeRefScrollToLi() -> Html {
    let list_ref: NodeRef = use_node_ref();
    let logo_colors: Vec<&str> = vec!["R", "G", "B"];

    let scroll_to_index: Callback<usize> = {
        let list_ref: NodeRef = list_ref.clone();
        let logo_colors: Vec<&str> = logo_colors.clone();
        Callback::from(move |index: usize| {
            if let Some(list_node) = list_ref.cast::<HtmlElement>() {
                if let Ok(li_nodes) = list_node.query_selector_all("li") {
                    if let Some(li_node) = li_nodes.item(index as u32) {
                        if let Some(li_element) = li_node.dyn_ref::<Element>() {
                            console::log_2(
                                &"Scrolling into view".into(),
                                &logo_colors[index].into(),
                            );
                            let mut options: ScrollIntoViewOptions = ScrollIntoViewOptions::new();
                            options.behavior(ScrollBehavior::Smooth);
                            li_element.scroll_into_view_with_scroll_into_view_options(&options);
                        }
                    }
                }
            }
        })
    };

    html! {
        <>
            <nav class="fixed top-4 left-1/2 -translate-x-1/2 flex flex-col sm:flex-row gap-4 w-max p-4 bg-gray-100 rounded-md">
            { for logo_colors.iter().enumerate().map(|(index, &color)| {
                let onclick = scroll_to_index.reform(move |_| index);
                html! {
                    <button
                        class="shrink-0 bg-white px-4 py-2 rounded-md transition-colors hover:text-white hover:bg-black duration-200"
                        {onclick}
                    >
                        {format!("Scroll to {}", color)}
                    </button>
                }
            }) }
            </nav>
            <div>
                <ul ref={list_ref} class="flex flex-col items-center justify-center w-full bg-gray-100">
                    <li class="flex items-center justify-center w-full h-[100dvh] bg-red-100">
                        <img src="assets/HereAfter-logo-r.svg" alt="HereAfter Logo (R)" />
                    </li>
                    <li class="flex items-center justify-center w-full h-[100dvh] bg-green-100">
                        <img src="assets/HereAfter-logo-g.svg" alt="HereAfter Logo (G)" />
                    </li>
                    <li class="flex items-center justify-center w-full h-[100dvh] bg-blue-100">
                        <img src="assets/HereAfter-logo-b.svg" alt="HereAfter Logo (B)" />
                    </li>
                </ul>
            </div>
        </>
    }
}

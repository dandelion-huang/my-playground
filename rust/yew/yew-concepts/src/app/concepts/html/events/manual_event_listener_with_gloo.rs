// compare with ./manual_event_listener_with_closure.rs

use gloo::{console::log, dialogs::alert, events::EventListener};
use web_sys::{CustomEvent, HtmlElement, KeyboardEvent};
use yew::prelude::*;

const MAX_SEQUENCE_LENGTH: usize = 20;

#[function_component]
fn MyComponent() -> Html {
    let div_node_ref: NodeRef = use_node_ref();
    let input_sequence: UseStateHandle<String> = use_state(String::new);
    let onkeypress: Callback<KeyboardEvent> = {
        let div_node_ref: NodeRef = div_node_ref.clone();
        let input_sequence: UseStateHandle<String> = input_sequence.clone();
        Callback::from(move |event: KeyboardEvent| {
            let key: String = event.key();
            let mut current_sequence: String = (*input_sequence).clone();
            current_sequence.push_str(&key);
            if current_sequence.ends_with("cupid") {
                if let Some(element) = div_node_ref.cast::<HtmlElement>() {
                    if element.id() == "cupid-div" {
                        log!("cupid-div is focused.");
                        // trigger oncupid event
                        let event: CustomEvent = web_sys::CustomEvent::new("cupid").unwrap();
                        element.dispatch_event(&event).unwrap();
                        log!("oncupid is triggered.");
                    }
                }
                current_sequence.clear();
            }
            if current_sequence.len() >= MAX_SEQUENCE_LENGTH {
                log!("Sequence too long, resetting.");
                current_sequence.clear();
            }
            input_sequence.set(current_sequence);
        })
    };

    use_effect_with(div_node_ref.clone(), {
        let div_node_ref: NodeRef = div_node_ref.clone();
        move |_| {
            // custom event: cupid
            let mut cupid_listener: Option<EventListener> = None;
            if let Some(element) = div_node_ref.cast::<HtmlElement>() {
                let oncupid: Callback<Event> = Callback::from(move |_: Event| {
                    alert("Love Love!");
                });
                // with gloo
                let listener: EventListener =
                    EventListener::new(&element, "cupid", move |event: &Event| {
                        oncupid.emit(event.clone())
                    });
                cupid_listener = Some(listener);
            }
            move || drop(cupid_listener)
        }
    });

    html! {
        <div class="flex flex-col gap-2 items-center justify-center w-full h-[100dvh] bg-gray-100">
            <h1 class="text-2xl">{"Please open the console."}</h1>
            <div class="px-24 py-10 bg-green-100 text-center" id="cupid-div" ref={div_node_ref} tabindex="0" {onkeypress}>
                {"CUPID!"}
                <br />
                {"just focus on!"}
                <br />
                {"and press 'c' 'u' 'p' 'i' 'd'!"}
            </div>
        </div>
    }
}

#[function_component]
pub fn ManualEventListenerWithGloo() -> Html {
    html! {
        <MyComponent />
    }
}

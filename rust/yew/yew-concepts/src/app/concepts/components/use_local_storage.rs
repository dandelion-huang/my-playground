use gloo::storage::{LocalStorage, Storage};
use yew::prelude::*;

#[hook]
pub fn use_local_storage<T>(key: &str, initial_value: T) -> (T, Callback<T>)
where
    T: 'static + Clone + serde::Serialize + serde::de::DeserializeOwned,
{
    let key: String = key.to_owned();
    let stored_value: T = LocalStorage::get(&key).unwrap_or_else(|_| initial_value.clone());
    let state: UseStateHandle<T> = use_state(|| stored_value);

    let setter: Callback<T> = {
        let key: String = key.clone();
        let state: UseStateHandle<T> = state.clone();
        Callback::from(move |value: T| {
            LocalStorage::set(&key, &value).expect("failed to set local storage");
            state.set(value);
        })
    };

    ((*state).clone(), setter)
}

use leptos::{component, prelude::*, view, IntoView};
use leptos_router::hooks::use_params_map;

#[component]
pub fn PackPage() -> impl IntoView {
    let params = use_params_map();

    /*     let data = Effect::new(async move |_| {
        let pack = params.read().get("id").unwrap();

    }) */

    view! {
        <h1>{params.get().get("id").unwrap()}</h1>
    }
}

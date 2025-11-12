use leptos::{component, prelude::*, task::spawn_local, view, IntoView};
use leptos_router::hooks::use_params_map;

use crate::{api::fetch_pack, types::Pack};

#[component]
pub fn PackPage() -> impl IntoView {
    let (pack, set_pack) = signal::<Option<Pack>>(None);
    let params = use_params_map();

    Effect::new(move |_| {
        let pack = params.read().get("id").unwrap();
        spawn_local(async move {let pack = fetch_pack(pack).await;
        set_pack.set(Some(pack.unwrap()));});
    });

    view! {
        {move || match pack.get() {
            Some(p) => view! { <h1>{p.mods[1].votes}</h1> },
            None => view! { <h1>Not found</h1> },
        }}
    }
}

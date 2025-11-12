use futures::{StreamExt, stream};
use leptos::{component, prelude::*, task::spawn_local, view, IntoView};
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;

use crate::{api::fetch_pack, types::FullMod};

#[component]
pub fn PackPage() -> impl IntoView {
    let (name, set_name) = signal::<Option<String>>(None);
    let (mods, set_mods) = signal::<Option<Vec<FullMod>>>(None);
    let params = use_params_map();

    Effect::new(move |_| {
        let pack = params.read().get("id").unwrap();
        spawn_local(async move {
            let pack = fetch_pack(pack).await.unwrap();
            set_name.set(Some(pack.name));
            set_mods.set(Some(stream::iter(pack.mods).filter_map(|m| FullMod::from_mod(m)).collect().await));
    })});

    view! {
        <Show
            when=move || name.get().is_some() && mods.get().is_some()
            fallback=|| view! { <h1>Not found</h1> }
        >
            <Title text=move || name.get().unwrap() />
            {move || {
                mods.get()
                    .unwrap()
                    .into_iter()
                    .map(|m| view! { <p>{m.description}</p> })
                    .collect_view()
            }}
        </Show>
    }
}

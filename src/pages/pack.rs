use leptos::{component, prelude::*, task::spawn_local, view, IntoView};
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;
use reqwest::get;
use std::collections::HashMap;

use crate::{
    api::{downvote, fetch_pack},
    types::{FullMod, WebMod},
};

#[component]
pub fn PackPage() -> impl IntoView {
    let (name, set_name) = signal::<Option<String>>(None);
    let (mods, set_mods) = signal::<Option<Vec<FullMod>>>(None);
    let params = use_params_map();

    Effect::new(move |_| {
        let pack = params.read().get("id").unwrap();
        spawn_local(async move {
            let pack = fetch_pack(pack).await.unwrap();
            if pack.mods.is_empty() {
                set_mods.set(Some(Vec::new()));
                return;
            }
            let mut votes = HashMap::new();
            for m in &pack.mods {
                votes.insert(m.slug.clone(), m.votes);
            }
            let web_mod: Vec<WebMod> = get(format!(
                "https://api.modrinth.com/v2/projects?ids=[\"{}\"]",
                pack.mods
                    .into_iter()
                    .map(|m| m.slug)
                    .collect::<Vec<String>>()
                    .join("\", \"")
            ))
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
            let mods = web_mod
                .into_iter()
                .map(|m| FullMod {
                    votes: votes.get(&m.slug).unwrap().clone(),
                    slug: m.slug,
                    title: m.title,
                    description: m.description,
                    icon: m.icon_url,
                })
                .collect();
            set_name.set(Some(pack.name));
            set_mods.set(Some(mods));
        })
    });

    view! {
        <Show
            when=move || name.get().is_some() && mods.get().is_some()
            fallback=|| view! { <h1>Not found</h1> }
        >
            <Title text=move || name.get().unwrap() />
            <ul>
                {move || {
                    mods.get()
                        .unwrap()
                        .into_iter()
                        .map(|m| {
                            view! {
                                <li>
                                    <img src=m.icon />
                                    <h1>{m.title}</h1>
                                    <p>{m.description}</p>
                                    <button>
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="24"
                                            height="24"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="#c0c0c0"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            class="lucide lucide-thumbs-up-icon lucide-thumbs-up"
                                        >
                                            <path d="M7 10v12" />
                                            <path d="M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2a3.13 3.13 0 0 1 3 3.88Z" />
                                        </svg>
                                    </button>
                                    <p>{m.votes}</p>
                                    <button on:click=move |_| {
                                        let slug = m.slug.clone();
                                        spawn_local(async {
                                            downvote(
                                                params.read().get("id").unwrap(),
                                                slug,
                                            ).await.unwrap()});}>
                                        <svg
                                            xmlns="http://www.w3.org/2000/svg"
                                            width="24"
                                            height="24"
                                            viewBox="0 0 24 24"
                                            fill="none"
                                            stroke="#c0c0c0"
                                            stroke-width="2"
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            class="lucide lucide-thumbs-down-icon lucide-thumbs-down"
                                        >
                                            <path d="M17 14V2" />
                                            <path d="M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22a3.13 3.13 0 0 1-3-3.88Z" />
                                        </svg>
                                    </button>
                                </li>
                            }
                        })
                        .collect_view()
                }}
            </ul>
        </Show>
    }
}

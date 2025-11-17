use leptos::{
    IntoView, component, prelude::*, task::spawn_local, view
};
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;
use reqwest::get;
use std::collections::HashMap;

use crate::{
    api::{add_mods, fetch_pack},
    types::{Dependencies, FullMod, ModSearch, WebMod},
};

#[component]
pub fn AddPage() -> impl IntoView {
    let (name, set_name) = signal::<Option<String>>(None);
    let (version, set_version) = signal::<Option<String>>(None);
    let (loader, set_loader) = signal::<Option<String>>(None);
    let (mods, set_mods) = signal::<Option<Vec<FullMod>>>(None);
    let (result_mods, set_result_mods) = signal(None);
    let (reload, force_reload) = signal(());
    let input = signal("".to_string());
    let params = use_params_map();

    Effect::new(move |_| {
        let _ = reload.get();
        let pack_str = params.read().get("id").unwrap();
        spawn_local(async move {
            let pack = fetch_pack(pack_str.clone()).await.unwrap();
            if pack.mods.is_empty() {
                set_name.set(Some(pack.name));
                set_version.set(Some(pack.version));
                set_loader.set(Some(pack.loader));
                set_mods.set(Some(Vec::new()));
                return;
            }
            let mut votes_tmp = HashMap::new();
            for m in &pack.mods {
                votes_tmp.insert(m.slug.clone(), m.votes);
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
            let mods: Vec<FullMod> = web_mod
                .into_iter()
                .map(|m| FullMod {
                    pack: pack_str.clone(),
                    votes: votes_tmp.get(&m.slug).unwrap().clone(),
                    slug: m.slug,
                    title: m.title,
                    description: m.description,
                    icon: m.icon_url,
                })
                .collect();
            set_name.set(Some(pack.name));
            set_version.set(Some(pack.version));
            set_loader.set(Some(pack.loader));
            set_mods.set(Some(mods));
        })
    });

    Effect::watch(
        move || input.0.get(),
        move |query, _, _| {
            if loader.get_untracked().is_none() && version.get_untracked().is_none() {
                return;
            }
            let query = query.clone();
            spawn_local(async move {
                let mods: ModSearch = get(
                        format!(
                            "https://api.modrinth.com/v2/search?limit=100&query={}&facets=[[\"project_type:mod\"],[\"categories:{}\"],[\"versions:{}\"]]",
                            query,
                            loader.get_untracked().unwrap(),
                            version.get_untracked().unwrap(),
                        ),
                    )
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                set_result_mods.set(Some(mods));
            })
        },
        false
    );

    view! {
        <Show
            when=move || name.get().is_some() && mods.get().is_some()
            fallback=|| view! { <h1>Not found</h1> }
        >
            <Title text=move || name.get().unwrap() />
            <div class="fixed z100 bg-gray-800 w-full h-20 flex items-center">
                <h1 class="text-xl font-bold pl-5 text-gray-100">{loader}</h1>
                <h1 class="text-xl font-bold pl-5 text-gray-100">{version}</h1>
                <input
                    class="p-2 ml-auto text-xl mr-5 bg-gray-600 text-gray-100"
                    placeholder="Search"
                    bind:value=input
                    type="text"
                />
                <a
                    class="text-xl pr-5 text-gray-100"
                    href=format!("/pack/{}", params.read().get("id").unwrap())
                >
                    Back
                </a>
            </div>
            <ul class="w-full h-full bg-gray-700 pt-20 text-gray-100">
                {move || {
                    let Some(mods) = mods.get() else { return Vec::new() };
                    let slugs: Vec<String> = mods.into_iter().map(|m| m.slug).collect();
                    let Some(results) = result_mods.get() else { return Vec::new() };
                    let pack = params.read().get("id").unwrap();
                    results
                        .hits
                        .into_iter()
                        .map(|m| {
                            view! {
                                <li class="grid grid-cols-[5rem_15%_1fr_15rem] h-20 bg-gray-600 mb-1">
                                    <img class="w-full h-full object-cover" src=m.icon_url />
                                    <h1 class="leading-20 text-center font-bold">{m.title}</h1>
                                    <p class="leading-20">{m.description}</p>
                                    {
                                        let slug = m.slug;
                                        let added = slugs.contains(&slug);
                                        view! {
                                            <button on:click={
                                                let slug = slug.clone();
                                                let pack = pack.clone();
                                                move |_| {
                                                    if added {
                                                        return;
                                                    }
                                                    let slug = slug.clone();
                                                    let pack = pack.clone();
                                                    spawn_local(async move {
                                                        let dependencies: Dependencies = get(
                                                                format!(
                                                                    "https://api.modrinth.com/v2/project/{}/dependencies",
                                                                    slug,
                                                                ),
                                                            )
                                                            .await
                                                            .unwrap()
                                                            .json()
                                                            .await
                                                            .unwrap();
                                                        let mut slugs: Vec<String> = dependencies
                                                            .projects
                                                            .into_iter()
                                                            .map(|m| m.slug)
                                                            .collect();
                                                        slugs.push(slug);
                                                        add_mods(pack, slugs).await.unwrap();
                                                        force_reload.set(());
                                                    });
                                                }
                                            }>{if added { "Added" } else { "Add" }}</button>
                                        }
                                    }
                                </li>
                            }
                        })
                        .collect_view()
                }}
            </ul>
        </Show>
    }
}

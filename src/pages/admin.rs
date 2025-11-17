use leptos::{
    IntoView, component, wasm_bindgen::JsCast, prelude::*, server::codee::string::FromToStringCodec, task::spawn_local, view, web_sys::HtmlElement
};
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;
use leptos_use::storage::use_local_storage;
use reqwest::get;
use std::collections::HashMap;

use crate::{
    api::{downvote, fetch_pack, remove_mod, remove_vote, upvote},
    types::{FullMod, WebMod},
};

#[component]
pub fn AdminPage() -> impl IntoView {
    let (name, set_name) = signal::<Option<String>>(None);
    let (mods, set_mods) = signal::<Option<Vec<FullMod>>>(None);
    let params = use_params_map();

    Effect::new(move |_| {
        let pack_str = params.read().get("id").unwrap();
        spawn_local(async move {
            let pack = fetch_pack(pack_str.clone()).await.unwrap();
            if pack.mods.is_empty() {
                set_name.set(Some(pack.name));
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
            set_mods.set(Some(mods));
        })
    });

    view! {
        <Show
            when=move || name.get().is_some() && mods.get().is_some()
            fallback=|| view! { <h1>Not found</h1> }
        >
            <Title text=move || name.get().unwrap() />
            <div class="fixed z100 bg-gray-800 w-full h-20 flex items-center">
                <h1 class="text-xl lpl-5 font-bold pl-5 text-gray-100">{name.get().unwrap()}</h1>
                <a
                    class="ml-auto text-xl pr-5 text-gray-100"
                    href=format!("/pack/{}/add", params.read().get("id").unwrap())
                >
                    Add mods
                </a>
            </div>
            <ul class="w-full h-full bg-gray-700 pt-20 text-gray-100">
                {move || {
                    mods.get()
                        .unwrap()
                        .into_iter()
                        .map(|m| {
                            let m2 = m.clone();
                            let m3 = m.clone();
                            let (vote, set_vote, _) = use_local_storage::<
                                i32,
                                FromToStringCodec,
                            >(format!("{}-{}", m.pack, m.slug));
                            let start_vote = vote.get_untracked();
                            let key = params.get().get("key").unwrap();
                            view! {
                                <li class="pr-5 grid grid-cols-[5rem_15%_1fr_2.5rem_2.5rem_2.5rem_5rem] h-20 bg-gray-600 mb-1">
                                    <img class="w-full h-full object-cover" src=m.icon />
                                    <h1 class="text-center p-2.5 overflow-y-scroll font-bold">
                                        {m.title}
                                    </h1>
                                    <p class="p-2.5 overflow-y-scroll">{m.description}</p>
                                    <button
                                        class="flex items-center justify-center w-full"
                                        on:click=move |_| {
                                            let slug = m.slug.clone();
                                            let pack_id = m.pack.clone();
                                            if vote.get_untracked() > 0 {
                                                set_vote.set(0);
                                                spawn_local(async {
                                                    remove_vote(pack_id, slug).await.unwrap();
                                                });
                                            } else {
                                                set_vote.set(1);
                                                spawn_local(async {
                                                    upvote(pack_id, slug).await.unwrap();
                                                });
                                            }
                                        }
                                    >
                                        {move || {
                                            if vote.get() > 0 {
                                                view! {
                                                    <svg
                                                        xmlns="http://www.w3.org/2000/svg"
                                                        width="24"
                                                        height="24"
                                                        viewBox="0 0 24 24"
                                                        fill="none"
                                                        stroke="#00ff00"
                                                        stroke-width="2"
                                                        stroke-linecap="round"
                                                        stroke-linejoin="round"
                                                        class="lucide lucide-thumbs-up-icon lucide-thumbs-up"
                                                    >
                                                        <path d="M7 10v12" />
                                                        <path d="M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H4a2 2 0 0 1-2-2v-8a2 2 0 0 1 2-2h2.76a2 2 0 0 0 1.79-1.11L12 2a3.13 3.13 0 0 1 3 3.88Z" />
                                                    </svg>
                                                }
                                            } else {
                                                view! {
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
                                                }
                                            }
                                        }}
                                    </button>
                                    <p class="leading-20 text-center">
                                        {move || m.votes - start_vote + vote.get()}
                                    </p>
                                    <button
                                        class="flex items-center justify-center w-full"
                                        on:click=move |_| {
                                            let slug = m2.slug.clone();
                                            let pack_id = m2.pack.clone();
                                            if vote.get_untracked() < 0 {
                                                set_vote.set(0);
                                                spawn_local(async {
                                                    remove_vote(pack_id, slug).await.unwrap()
                                                });
                                            } else {
                                                set_vote.set(-1);
                                                spawn_local(async {
                                                    downvote(pack_id, slug).await.unwrap()
                                                });
                                            }
                                        }
                                    >
                                        {move || {
                                            if vote.get() < 0 {
                                                view! {
                                                    <svg
                                                        xmlns="http://www.w3.org/2000/svg"
                                                        width="24"
                                                        height="24"
                                                        viewBox="0 0 24 24"
                                                        fill="none"
                                                        stroke="#ff0000"
                                                        stroke-width="2"
                                                        stroke-linecap="round"
                                                        stroke-linejoin="round"
                                                        class="lucide lucide-thumbs-down-icon lucide-thumbs-down"
                                                    >
                                                        <path d="M17 14V2" />
                                                        <path d="M9 18.12 10 14H4.17a2 2 0 0 1-1.92-2.56l2.33-8A2 2 0 0 1 6.5 2H20a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-2.76a2 2 0 0 0-1.79 1.11L12 22a3.13 3.13 0 0 1-3-3.88Z" />
                                                    </svg>
                                                }
                                            } else {
                                                view! {
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
                                                }
                                            }
                                        }}
                                    </button>
                                    <button
                                        class="leading-20 text-center"
                                        on:click=move |ev| {
                                            let target = ev.current_target().unwrap();
                                            let element = target.dyn_ref::<HtmlElement>().unwrap();
                                            let parent = element.parent_element().unwrap();
                                            parent.remove();
                                            let pack = m3.pack.clone();
                                            let slug = m3.slug.clone();
                                            let key = key.clone();
                                            spawn_local(async {
                                                remove_mod(pack, key, slug).await.unwrap();
                                            })
                                        }
                                    >
                                        Delete
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

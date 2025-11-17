use leptos::{component, logging::log, prelude::*, reactive::spawn_local, view, IntoView};

use crate::api::create_pack;

#[component]
pub fn HomePage() -> impl IntoView {
    let name = signal(String::new());
    let loader = signal(String::new());
    let version = signal(String::new());

    view! {
        <div class="w-full h-full flex items-center justify-center">
            <div class="grid grid-cols-2 gap-4">
                <input
                    class="text-gray-100 bg-gray-800 col-span-2 px-4 py-2 rounded"
                    bind:value=name
                    type="text"
                    placeholder="Name"
                />
                <select
                    class="text-gray-100 bg-gray-800 text-center px-4 py-2 rounded"
                    bind:value=loader
                >
                    <option selected value="fabric">
                        Fabric
                    </option>
                    <option value="forge">Forge</option>
                    <option value="neoforge">NeoForge</option>
                    <option value="quilt">Quilt</option>
                </select>
                <select
                    class="text-gray-100 bg-gray-800 text-center px-4 py-2 rounded"
                    bind:value=version
                >
                    <option value="1.21.10">1.21.10</option>
                    <option value="1.21.9">1.21.9</option>
                    <option value="1.21.8">1.21.8</option>
                    <option value="1.21.7">1.21.7</option>
                    <option value="1.21.6">1.21.6</option>
                    <option value="1.21.5">1.21.5</option>
                    <option value="1.21.4">1.21.4</option>
                    <option value="1.21.3">1.21.3</option>
                    <option value="1.21.2">1.21.2</option>
                    <option value="1.21.1">1.21.1</option>
                    <option value="1.21">1.21</option>
                    <option value="1.20.6">1.20.6</option>
                    <option value="1.20.5">1.20.5</option>
                    <option value="1.20.4">1.20.4</option>
                    <option value="1.20.3">1.20.3</option>
                    <option value="1.20.2">1.20.2</option>
                    <option value="1.20.1">1.20.1</option>
                    <option value="1.20">1.20</option>
                    <option value="1.19.4">1.19.4</option>
                    <option value="1.19.3">1.19.3</option>
                    <option value="1.19.2">1.19.2</option>
                    <option value="1.19.1">1.19.1</option>
                    <option value="1.19">1.19</option>
                    <option value="1.18.2">1.18.2</option>
                    <option value="1.18.1">1.18.1</option>
                    <option value="1.18">1.18</option>
                    <option value="1.17.1">1.17.1</option>
                    <option value="1.17">1.17</option>
                    <option value="1.16.5">1.16.5</option>
                    <option value="1.16.4">1.16.4</option>
                    <option value="1.16.3">1.16.3</option>
                    <option value="1.16.2">1.16.2</option>
                    <option value="1.16.1">1.16.1</option>
                    <option value="1.16">1.16</option>
                    <option value="1.15.2">1.15.2</option>
                    <option value="1.15.1">1.15.1</option>
                    <option value="1.15">1.15</option>
                    <option value="1.14.4">1.14.4</option>
                    <option value="1.14.3">1.14.3</option>
                    <option value="1.14.2">1.14.2</option>
                    <option value="1.14.1">1.14.1</option>
                    <option value="1.14">1.14</option>
                    <option value="1.13.2">1.13.2</option>
                    <option value="1.13.1">1.13.1</option>
                    <option value="1.13">1.13</option>
                    <option value="1.12.2">1.12.2</option>
                    <option value="1.12.1">1.12.1</option>
                    <option value="1.12">1.12</option>
                    <option value="1.11.2">1.11.2</option>
                    <option value="1.11.1">1.11.1</option>
                    <option value="1.11">1.11</option>
                    <option value="1.10.2">1.10.2</option>
                    <option value="1.10.1">1.10.1</option>
                    <option value="1.10">1.10</option>
                    <option value="1.9.4">1.9.4</option>
                    <option value="1.9.3">1.9.3</option>
                    <option value="1.9.2">1.9.2</option>
                    <option value="1.9.1">1.9.1</option>
                    <option value="1.9">1.9</option>
                    <option value="1.8.9">1.8.9</option>
                    <option value="1.8.8">1.8.8</option>
                    <option value="1.8.7">1.8.7</option>
                    <option value="1.8.6">1.8.6</option>
                    <option value="1.8.5">1.8.5</option>
                    <option value="1.8.4">1.8.4</option>
                    <option value="1.8.3">1.8.3</option>
                    <option value="1.8.2">1.8.2</option>
                    <option value="1.8.1">1.8.1</option>
                    <option value="1.8">1.8</option>
                    <option value="1.7.10">1.7.10</option>
                    <option value="1.7.9">1.7.9</option>
                    <option value="1.7.8">1.7.8</option>
                    <option value="1.7.7">1.7.7</option>
                    <option value="1.7.6">1.7.6</option>
                    <option value="1.7.5">1.7.5</option>
                    <option value="1.7.4">1.7.4</option>
                    <option value="1.7.3">1.7.3</option>
                    <option value="1.7.2">1.7.2</option>
                    <option value="1.6.4">1.6.4</option>
                    <option value="1.6.2">1.6.2</option>
                    <option value="1.6.1">1.6.1</option>
                    <option value="1.5.2">1.5.2</option>
                    <option value="1.5.1">1.5.1</option>
                    <option value="1.4.7">1.4.7</option>
                    <option value="1.4.6">1.4.6</option>
                    <option value="1.4.5">1.4.5</option>
                    <option value="1.4.4">1.4.4</option>
                    <option value="1.4.2">1.4.2</option>
                    <option value="1.3.2">1.3.2</option>
                    <option value="1.3.1">1.3.1</option>
                    <option value="1.2.5">1.2.5</option>
                    <option value="1.2.4">1.2.4</option>
                    <option value="1.2.3">1.2.3</option>
                    <option value="1.2.2">1.2.2</option>
                    <option value="1.2.1">1.2.1</option>
                    <option value="1.1">1.1</option>
                    <option value="1.0">1.0</option>
                </select>
                <button
                    class="text-gray-100 col-span-2 px-4 py-2 rounded bg-gray-800"
                    on:click=move |_| {
                        if name.0.get_untracked().is_empty() || loader.0.get_untracked().is_empty()
                            || version.0.get_untracked().is_empty()
                        {
                            log!("returning");
                            return;
                        }
                        spawn_local(async move {
                            create_pack(
                                    name.0.get_untracked(),
                                    loader.0.get_untracked(),
                                    version.0.get_untracked(),
                                )
                                .await
                                .unwrap();
                        });
                    }
                >
                    Create Pack
                </button>
            </div>
        </div>
    }
}

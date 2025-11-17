use crate::pages::{AddPage, AdminPage, HomePage, PackPage};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path, StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" class="h-full bg-gray-700">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <Stylesheet id="leptos" href="/pkg/modpack.css" />
                <MetaTags />
            </head>
            <body class="h-full">
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="modpack creator" />
        <Router>
            <main class="h-full">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=path!("/pack/:id") view=PackPage />
                    <Route path=path!("/pack/:id/add") view=AddPage />
                    <Route path=path!("/pack/:id/:key") view=AdminPage />
                </Routes>
            </main>
        </Router>
    }
}

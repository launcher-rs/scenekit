use leptos::prelude::*;

const CRATES: &[&str] = &[
    "scenekit",
    "scenekit-math",
    "scenekit-core",
    "scenekit-scene",
    "scenekit-camera",
    "scenekit-mesh",
    "scenekit-material",
    "scenekit-light",
    "scenekit-texture",
    "scenekit-renderer",
    "scenekit-loader",
    "scenekit-post",
    "scenekit-raycaster",
    "scenekit-helpers",
    "scenekit-animato",
    "scenekit-wasm",
    "scenekit-input",
];

#[component]
pub fn CrateMap() -> impl IntoView {
    view! {
        <section class="section" id="crates">
            <p class="eyebrow">"Crate Map"</p>
            <h2>"Composable by default"</h2>
            <div class="crate-map">
                {CRATES.iter().map(|name| view! {
                    <a href=format!("https://crates.io/crates/{name}")>{*name}</a>
                }).collect_view()}
            </div>
        </section>
    }
}

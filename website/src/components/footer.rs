use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <span>"Aarambh Dev Hub"</span>
            <nav aria-label="Footer links">
                <a href="https://github.com/launcher-rs/scenekit">"GitHub"</a>
                <a href="https://crates.io/crates/scenekit">"crates.io"</a>
                <a href="https://docs.rs/scenekit">"docs.rs"</a>
                <span>"MIT OR Apache-2.0"</span>
            </nav>
        </footer>
    }
}

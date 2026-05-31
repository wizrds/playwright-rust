use leptos::prelude::*;

use super::CodeBlock;

const CARGO_TOML: &str = r#"[dependencies]
playwright-rs = "0.13"           # auto-updates within 0.13.x
tokio = { version = "1", features = ["full"] }"#;

#[component]
pub fn Install() -> impl IntoView {
    view! {
        <section id="install" class="mx-auto max-w-3xl px-6 py-12">
            <h2 class="mb-4 text-2xl font-bold text-rust-300">"Install"</h2>
            <CodeBlock code=CARGO_TOML/>
            <p class="mt-4 text-sm text-rust-50/70">
                "Default feature "
                <code class="text-rust-300">"macros"</code>
                " ships the compile-time "
                <code class="text-rust-300">"locator!()"</code>
                " selector macro. Opt in to "
                <code class="text-rust-300">"cli"</code>
                " (the browser-installer binary) and "
                <code class="text-rust-300">"screenshot-diff"</code>
                " (pixel-diff assertions) as needed."
            </p>
        </section>
    }
}

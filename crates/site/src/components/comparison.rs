use leptos::prelude::*;

use super::CodeBlock;

const PYTHON: &str = r#"from playwright.sync_api import sync_playwright

with sync_playwright() as p:
    browser = p.chromium.launch()
    page = browser.new_page()
    page.goto("https://example.com")

    heading = page.locator("h1")
    assert heading.text_content() == "Example Domain"

    browser.close()"#;

const RUST: &str = r#"use playwright_rs::Playwright;

let pw = Playwright::launch().await?;
let browser = pw.chromium().launch().await?;
let page = browser.new_page().await?;
page.goto("https://example.com", None).await?;

let heading = page.locator("h1").await;
assert_eq!(heading.text_content().await?, Some("Example Domain".into()));

browser.close().await?;"#;

#[component]
pub fn Comparison() -> impl IntoView {
    view! {
        <section id="comparison" class="mx-auto max-w-5xl px-6 py-12">
            <h2 class="mb-2 text-2xl font-bold text-rust-300">"Familiar from day one"</h2>
            <p class="mb-6 max-w-2xl text-sm text-rust-50/70">
                "The API matches Playwright's cross-language conventions — if you know "
                "playwright-python, you know playwright-rs."
            </p>
            <div class="grid grid-cols-1 gap-6 md:grid-cols-2">
                <CodeBlock caption="Python" code=PYTHON/>
                <CodeBlock caption="Rust" code=RUST/>
            </div>
        </section>
    }
}

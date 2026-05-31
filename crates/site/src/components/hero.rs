use leptos::prelude::*;

const CRATES_IO: &str = "https://crates.io/crates/playwright-rs";
const DOCS_RS: &str = "https://docs.rs/playwright-rs";
const GITHUB: &str = "https://github.com/padamson/playwright-rust";

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <header class="flex flex-col items-center px-6 pt-24 pb-16 text-center">
            <h1
                id="hero-title"
                class="text-5xl font-bold tracking-tight text-rust-500 sm:text-6xl"
            >
                "Playwright for Rust"
            </h1>
            <p id="hero-tagline" class="mt-5 max-w-2xl text-lg text-rust-50/80">
                "Cross-browser end-to-end testing for Rust — official-quality bindings for "
                "Microsoft Playwright, with the same API you already know from Python, Java, and .NET."
            </p>

            <div class="mt-7 flex flex-wrap items-center justify-center gap-2">
                <a href=CRATES_IO>
                    <img alt="crates.io" src="https://img.shields.io/crates/v/playwright-rs.svg"/>
                </a>
                <a href=DOCS_RS>
                    <img alt="docs.rs" src="https://docs.rs/playwright-rs/badge.svg"/>
                </a>
                <a href=GITHUB>
                    <img
                        alt="CI"
                        src="https://github.com/padamson/playwright-rust/actions/workflows/test.yml/badge.svg"
                    />
                </a>
                <img
                    alt="Playwright 1.60.0"
                    src="https://img.shields.io/badge/Playwright-1.60.0-45ba4b"
                />
            </div>

            <div class="mt-9 flex flex-wrap items-center justify-center gap-3">
                <a
                    id="cta-get-started"
                    href=DOCS_RS
                    class="rounded-lg bg-rust-500 px-5 py-2.5 font-semibold text-rust-50 transition hover:bg-rust-600"
                >
                    "Get started"
                </a>
                <a
                    href=GITHUB
                    class="rounded-lg border border-rust-700/50 px-5 py-2.5 font-semibold text-rust-50 transition hover:border-rust-500"
                >
                    "View on GitHub"
                </a>
                <a
                    href=CRATES_IO
                    class="rounded-lg border border-rust-700/50 px-5 py-2.5 font-semibold text-rust-50 transition hover:border-rust-500"
                >
                    "crates.io"
                </a>
            </div>
        </header>
    }
}

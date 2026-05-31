use leptos::prelude::*;

/// A syntax-neutral code block with an optional caption (e.g. "Python" / "Rust").
#[component]
pub fn CodeBlock(
    /// The code to display, verbatim.
    code: &'static str,
    /// Optional label shown above the block.
    #[prop(optional, into)]
    caption: Option<&'static str>,
) -> impl IntoView {
    view! {
        <div class="flex flex-col">
            {caption.map(|c| view! {
                <span class="mb-2 text-xs font-semibold uppercase tracking-wider text-rust-300">
                    {c}
                </span>
            })}
            <pre class="overflow-x-auto rounded-lg border border-rust-700/40 bg-ink-800 p-4 text-sm leading-relaxed text-rust-50/90">
                <code>{code}</code>
            </pre>
        </div>
    }
}

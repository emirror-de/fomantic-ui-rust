use leptos::*;

/// A simple label.
#[component]
pub fn Label(text: MaybeSignal<String>) -> impl IntoView {
    view! {
        <label>
            { text }
        </label>
    }
}

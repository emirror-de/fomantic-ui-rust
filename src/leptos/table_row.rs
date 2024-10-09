use leptos::*;

/// Defines a row in a `fomantic-ui` table.
#[component]
pub fn TableRow(children: Children) -> impl IntoView {
    view! {
        <tr>
            { children() }
        </tr>
    }
}

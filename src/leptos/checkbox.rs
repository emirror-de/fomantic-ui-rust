use crate::models::Selectable;
use leptos::{
    html::ElementDescriptor,
    *,
};

/// A checkbox with data attached.
#[component]
#[allow(unused_braces)]
pub fn Checkbox<D, El>(
    checkbox_wrapper: Box<dyn Fn() -> HtmlElement<El>>,
    data: RwSignal<D>,
) -> impl IntoView
where
    D: Selectable + 'static,
    El: ElementDescriptor + 'static,
{
    // this might be optimized at a later stage,
    // `on_change` triggers `is_checked` but it is not necessary
    // but if this is untracked, it is not triggered from the outside anymore
    let is_checked = move || data.with(|d| d.is_selected());

    let on_change = move |e: web_sys::Event| {
        data.update(|d| {
            if event_target_checked(&e) {
                d.select();
            } else {
                d.deselect();
            }
        });
    };

    let input_view = view! {
        <input
            prop:checked=is_checked
            type="checkbox"
            on:change=on_change
            />
    };
    let checkbox_wrapper =
        checkbox_wrapper().child(input_view).classes("ui checkbox");

    view! {
        { checkbox_wrapper }
    }
}

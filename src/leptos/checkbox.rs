use crate::models::SelectableData;
use leptos::{
    html::ElementDescriptor,
    *,
};

/// A checkbox with data attached.
#[component]
#[allow(unused_braces)]
pub fn Checkbox<D, El>(
    checkbox_wrapper: Box<dyn Fn() -> HtmlElement<El>>,
    data: RwSignal<SelectableData<D>>,
    #[prop(optional)] label_wrapper: Option<Box<dyn Fn() -> HtmlElement<El>>>,
    label_fn: Box<dyn Fn(&D) -> String>,
) -> impl IntoView
where
    D: 'static,
    El: ElementDescriptor + 'static,
{
    // this might be optimized at a later stage,
    // `on_change` triggers `is_checked` but it is not necessary
    // but if this is untracked, it is not triggered from the outside anymore
    let is_checked = move || data.with(|d| d.is_selected);

    let on_change = move |e: web_sys::Event| {
        data.update(|d| {
            d.is_selected = event_target_checked(&e);
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

    let label_view = view! {
        <label>
        { move || data.with(|d| label_fn(&d.data))  }
        </label>
    };
    let label_wrapper = if let Some(wrapper) = label_wrapper {
        wrapper().child(label_view).into_any()
    } else {
        label_view.into_any()
    };

    view! {
        { checkbox_wrapper }
        { label_wrapper }
    }
}

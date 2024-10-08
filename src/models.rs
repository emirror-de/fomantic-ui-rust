//! Intermediary models to be used with different components.
#[cfg(feature = "leptos")]
use leptos::RwSignal;

/// Contains data with an additional field wether it has been selected or not.
pub struct SelectableData<D> {
    /// Wether the data has been selected.
    pub is_selected: bool,
    /// The actual data.
    pub data: D,
}

impl<D> SelectableData<D> {
    /// Creates a new instance, wrapped into an [RwSignal].
    #[cfg(feature = "leptos")]
    pub fn new(data: D, is_selected: bool) -> RwSignal<Self> {
        RwSignal::new(Self { is_selected, data })
    }
}

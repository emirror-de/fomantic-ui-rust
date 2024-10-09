//! Intermediary models to be used with different components.

/// Implementors get the ability to be selected, unselected or toggled.
/// Useful for eg. checkboxes with data attached.
pub trait Selectable {
    /// Selects the data.
    fn select(&mut self);
    /// Unselects the data.
    fn deselect(&mut self);
    /// Toggles the selection of the data.
    fn toggle(&mut self);
    /// Returns the current selection state.
    fn is_selected(&self) -> bool;
}

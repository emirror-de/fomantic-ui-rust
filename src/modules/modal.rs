//! Modal bindings.

use anyhow::anyhow;
use gloo::utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// [Modal] action configuration.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModalAction {
    /// Text of the action.
    pub text: String,
    /// The class of the action element.
    pub class: String,
}

/// [Modal] configuration options.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModalConfig {
    /// Title of the modal.
    pub title: String,
    /// Class to be added to the modal.
    pub class: String,
    /// Whether a close icon should be shown.
    pub close_icon: bool,
    /// The content of the modal.
    pub content: String,
    /// Available actions.
    pub actions: Vec<ModalAction>,
}

#[wasm_bindgen]
extern "C" {
    /// A modal.
    pub type Modal;

    /// Internal function to create the modal on JavaScript side.
    #[wasm_bindgen(js_namespace=["$"], js_name="modal")]
    fn new_modal(props: &JsValue) -> Modal;

    /// Internal function to create the modal on JavaScript side by using a selector.
    #[wasm_bindgen(js_name = "$", catch)]
    fn new_modal_from_selector(selector: &JsValue) -> Result<Modal, JsValue>;

    #[wasm_bindgen(method, js_name = "modal")]
    pub fn modal(this: &Modal, behavior: &JsValue);

    #[wasm_bindgen(method, js_name = "modal")]
    pub fn modal_returns_bool(this: &Modal, behavior: &JsValue) -> bool;
}

impl Modal {
    /// Creates a new modal.
    pub fn new(config: &ModalConfig) -> anyhow::Result<Self> {
        let modal_config_js =
            <JsValue as JsValueSerdeExt>::from_serde(&config).map_err(|e| anyhow!(e))?;
        Ok(new_modal(&modal_config_js))
    }

    /// Queries the modal by the given selector.
    pub fn query_from_selector(selector: &str) -> anyhow::Result<Self> {
        Ok(new_modal_from_selector(&JsValue::from(selector))
            .map_err(|e| anyhow!(format!("{e:#?}")))?)
    }

    /// Shows the modal.
    pub fn show(&self) {
        self.modal(&JsValue::from("show"));
    }

    /// Hides the modal.
    pub fn hide(&self) {
        self.modal(&JsValue::from("hide"));
    }

    /// Toggles the modal.
    pub fn toggle(&self) {
        self.modal(&JsValue::from("toggle"));
    }

    /// Refreshes centering of modal on page.
    pub fn refresh(&self) {
        self.modal(&JsValue::from("refresh"));
    }

    /// Shows associated page dimmer.
    pub fn show_dimmer(&self) {
        self.modal(&JsValue::from("show dimmer"));
    }

    /// Hides associated page dimmer.
    pub fn hide_dimmer(&self) {
        self.modal(&JsValue::from("hide dimmer"));
    }

    /// Hides all modals not selected modal in a dimmer.
    pub fn hide_others(&self) {
        self.modal(&JsValue::from("hide others"));
    }

    /// Hides all visible modals in the same dimmer.
    pub fn hide_all(&self) {
        self.modal(&JsValue::from("hide all"));
    }

    /// Caches current modal size.
    pub fn cache_sizes(&self) {
        self.modal(&JsValue::from("cache sizes"));
    }

    /// Returns whether the modal can fit on the page.
    pub fn can_fit(&self) -> bool {
        self.modal_returns_bool(&JsValue::from("can fit"))
    }

    /// Returns whether the modal is active.
    pub fn is_active(&self) -> bool {
        self.modal_returns_bool(&JsValue::from("is active"))
    }

    /// Sets modal to active.
    pub fn set_active(&self) {
        self.modal(&JsValue::from("set active"));
    }

    /// Destroys instance and removes all events.
    pub fn destroy(&self) {
        self.modal(&JsValue::from("destroy"));
    }
}

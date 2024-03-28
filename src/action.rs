/// Configuration for a Action.
use wasm_bindgen::prelude::*;

/// Defines an action that can be used in eg. [Modal](crate::modules::modal::Modal) or [Toast](crate::modules::toast::Toast).
pub struct Action {
    pub(crate) click: Closure<dyn Fn() -> bool>,
    pub(crate) js_config: JsActionConfig,
}

impl Action {
    /// Creates a new Action instance.
    pub fn new() -> Self {
        let js_config = JsActionConfig::new();
        let click = Closure::new(|| true);
        Self { js_config, click }
    }

    /// Sets the text shown on the action.
    pub fn with_text(self, text: &str) -> Self {
        self.js_config.set_text(text);
        self
    }

    /// Sets the CSS class name of the action.
    pub fn with_class(self, class: &str) -> Self {
        self.js_config.set_class(class);
        self
    }

    /// Sets the icon of the action.
    pub fn with_icon(self, icon: &str) -> Self {
        self.js_config.set_icon(icon);
        self
    }

    /// Sets the handler that is fired on click.
    pub fn click<H: Fn() -> bool + 'static>(mut self, click: H) -> Self {
        self.click = Closure::new(click);
        self.js_config.set_click(&self.click);
        self
    }
}

#[wasm_bindgen]
extern "C" {

    /// The JavaScript configuration object for toast actions.
    #[wasm_bindgen(js_name = Object)]
    pub(crate) type JsActionConfig;

    /// Configuration constructor for toast actions.
    #[wasm_bindgen(constructor, js_class = Object)]
    pub(crate) fn new() -> JsActionConfig;

    /// Set the text of the action.
    #[wasm_bindgen(method, setter, js_name = "text")]
    pub(crate) fn set_text(this: &JsActionConfig, text: &str);

    /// Set the CSS class of the action.
    #[wasm_bindgen(method, setter, js_name = "class")]
    pub(crate) fn set_class(this: &JsActionConfig, class: &str);

    /// Set the icon of the action.
    #[wasm_bindgen(method, setter, js_name = "icon")]
    pub(crate) fn set_icon(this: &JsActionConfig, icon: &str);

    /// Set the click handler.
    #[wasm_bindgen(method, setter, js_name = "click")]
    pub(crate) fn set_click(this: &JsActionConfig, click: &Closure<dyn Fn() -> bool>);

}

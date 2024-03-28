//! Modal bindings.
use crate::action::{
    Action,
    JsActionConfig,
};
use wasm_bindgen::prelude::*;

/// The configuration of a modal.
pub struct ModalConfig {
    pub(crate) js_config: JsModalConfig,
    on_show: Closure<dyn Fn() -> bool>,
    on_visible: Closure<dyn Fn() -> bool>,
    on_hide: Closure<dyn Fn(JsValue) -> bool>,
    on_hidden: Closure<dyn Fn() -> bool>,
    on_approve: Closure<dyn Fn(JsValue) -> bool>,
    on_deny: Closure<dyn Fn(JsValue) -> bool>,
}

impl ModalConfig {
    /// Is called when a modal starts to show. If the function returns false, the modal will not be shown.
    pub fn set_on_show<H: Fn() -> bool + 'static>(&mut self, handler: H) {
        self.on_show = Closure::new(handler);
        self.js_config.set_on_show(&self.on_show);
    }

    /// Is called after a modal has finished showing animating.
    pub fn set_on_visible<H: Fn() -> bool + 'static>(&mut self, handler: H) {
        self.on_visible = Closure::new(handler);
        self.js_config.set_on_visible(&self.on_visible);
    }

    /// Is called after a modal starts to hide. If the function returns false, the modal will not hide.
    pub fn set_on_hide<H: Fn(JsValue) -> bool + 'static>(
        &mut self,
        handler: H,
    ) {
        self.on_hide = Closure::new(handler);
        self.js_config.set_on_hide(&self.on_hide);
    }

    /// Is called after a modal has finished hiding animation.
    pub fn set_on_hidden<H: Fn() -> bool + 'static>(&mut self, handler: H) {
        self.on_hidden = Closure::new(handler);
        self.js_config.set_on_hidden(&self.on_hidden);
    }

    /// Is called after a positive, approve or ok button is pressed. If the function returns false, the modal will not hide.
    pub fn set_on_approve<H: Fn(JsValue) -> bool + 'static>(
        &mut self,
        handler: H,
    ) {
        self.on_approve = Closure::new(handler);
        self.js_config.set_on_approve(&self.on_approve);
    }

    /// Is called after a negative, deny or cancel button is pressed. If the function returns false the modal will not hide.
    pub fn set_on_deny<H: Fn(JsValue) -> bool + 'static>(
        &mut self,
        handler: H,
    ) {
        self.on_deny = Closure::new(handler);
        self.js_config.set_on_deny(&self.on_deny);
    }
}

impl Default for ModalConfig {
    fn default() -> Self {
        Self {
            js_config: JsModalConfig::new(),
            on_show: Closure::new(|| true),
            on_visible: Closure::new(|| true),
            on_hide: Closure::new(|_| true),
            on_hidden: Closure::new(|| true),
            on_approve: Closure::new(|_| true),
            on_deny: Closure::new(|_| true),
        }
    }
}

impl std::ops::Deref for ModalConfig {
    type Target = JsModalConfig;
    fn deref(&self) -> &Self::Target {
        &self.js_config
    }
}

/// A modal.
#[allow(unused)]
pub struct Modal {
    js_modal: JsModal,
    modal_config: ModalConfig,
    action_handler_list: Vec<Closure<dyn Fn() -> bool>>,
    alert_handler: Option<Closure<dyn Fn()>>,
    confirm_handler: Option<Closure<dyn Fn(bool)>>,
    prompt_handler: Option<Closure<dyn Fn(Option<String>)>>,
}

impl Default for Modal {
    fn default() -> Self {
        let modal_config = ModalConfig::default();
        Self {
            js_modal: new_modal(&modal_config),
            modal_config,
            action_handler_list: vec![],
            alert_handler: None,
            confirm_handler: None,
            prompt_handler: None,
        }
    }
}

impl Modal {
    /// Creates a new modal.
    pub fn new(modal_config: ModalConfig) -> Self {
        Self {
            js_modal: new_modal(&modal_config),
            modal_config,
            ..Default::default()
        }
    }

    /// Creates an `Alert` modal.
    pub fn new_alert<H>(title: &str, content: &str, handler: H) -> Self
    where
        H: Fn() + 'static,
    {
        let handler = Closure::new(handler);
        let js_modal = new_modal_alert("alert", title, content, &handler);
        Self {
            js_modal,
            modal_config: ModalConfig::default(),
            alert_handler: Some(handler),
            ..Default::default()
        }
    }

    /// Creates a `Confirm` modal.
    pub fn new_confirm<H>(title: &str, content: &str, handler: H) -> Self
    where
        H: Fn(bool) + 'static,
    {
        let handler = Closure::new(handler);
        let js_modal = new_modal_confirm("confirm", title, content, &handler);
        Self {
            js_modal,
            modal_config: ModalConfig::default(),
            confirm_handler: Some(handler),
            ..Default::default()
        }
    }

    /// Creates a `Prompt` modal.
    pub fn new_prompt<H: 'static>(
        title: &str,
        content: &str,
        handler: H,
    ) -> Self
    where
        H: Fn(Option<String>),
    {
        let handler = Closure::new(handler);
        let js_modal = new_modal_prompt("prompt", title, content, &handler);
        Self {
            js_modal,
            modal_config: ModalConfig::default(),
            prompt_handler: Some(handler),
            ..Default::default()
        }
    }

    /// Sets the title of the modal.
    pub fn with_title(self, title: &str) -> Self {
        self.modal_config.set_title(title);
        self
    }

    /// Sets the content of the modal.
    pub fn with_content(self, value: &str) -> Self {
        self.modal_config.set_content(value);
        self
    }

    /// Sets the title of the modal.
    pub fn with_class(self, class: &str) -> Self {
        self.modal_config.set_class(class);
        self
    }

    /// Wether a close icon should be shown.
    pub fn with_close_icon(self, value: bool) -> Self {
        self.modal_config.set_close_icon(value);
        self
    }

    /// Sets the actions shown on the modal.
    pub fn with_actions(mut self, actions: Vec<Action>) -> Self {
        let mut js_actions = vec![];
        for act in actions {
            self.action_handler_list.push(act.click);
            js_actions.push(act.js_config);
        }
        self.modal_config
            .js_config
            .set_actions(js_actions.into_boxed_slice());
        self
    }

    /// Shows the modal.
    pub fn show(&self) {
        self.js_modal.modal("show");
    }

    /// Hides the modal.
    pub fn hide(&self) {
        self.js_modal.modal("hide");
    }

    /// Toggles the modal.
    pub fn toggle(&self) {
        self.js_modal.modal("toggle");
    }

    /// Refreshes centering of modal on page.
    pub fn refresh(&self) {
        self.js_modal.modal("refresh");
    }

    /// Shows associated page dimmer.
    pub fn show_dimmer(&self) {
        self.js_modal.modal("show dimmer");
    }

    /// Hides associated page dimmer.
    pub fn hide_dimmer(&self) {
        self.js_modal.modal("hide dimmer");
    }

    /// Hides all modals not selected modal in a dimmer.
    pub fn hide_others(&self) {
        self.js_modal.modal("hide others");
    }

    /// Hides all visible modals in the same dimmer.
    pub fn hide_all(&self) {
        self.js_modal.modal("hide all");
    }

    /// Caches current modal size.
    pub fn cache_sizes(&self) {
        self.js_modal.modal("cache sizes");
    }

    /// Returns whether the modal can fit on the page.
    pub fn can_fit(&self) -> bool {
        self.js_modal.modal_returns_bool("can fit")
    }

    /// Returns whether the modal is active.
    pub fn is_active(&self) -> bool {
        self.js_modal.modal_returns_bool("is active")
    }

    /// Sets modal to active.
    pub fn set_active(&self) {
        self.js_modal.modal("set active");
    }

    /// Destroys instance and removes all events.
    pub fn destroy(&self) {
        self.js_modal.modal("destroy");
    }
}

#[wasm_bindgen]
extern "C" {

    /// The JavaScript configuration object for a modal.
    #[wasm_bindgen(js_name = Object)]
    pub type JsModalConfig;

    /// Configuration constructor for toast actions.
    #[wasm_bindgen(constructor, js_class = Object)]
    pub fn new() -> JsModalConfig;

    /// If set to false will prevent the modal from being moved to inside the dimmer.
    #[wasm_bindgen(method, setter)]
    pub fn set_detachable(this: &JsModalConfig, value: bool);

    /// Auto will automatically use flex in browsers that support absolutely positioned elements inside flex containers. Setting to true/false will force this setting for all browsers.
    #[wasm_bindgen(method, setter)]
    pub fn set_use_flex(this: &JsModalConfig, use_flex: JsValue);

    /// When true, the first form input inside the modal will receive focus when shown. Set this to false to prevent this behavior.
    #[wasm_bindgen(method, setter)]
    pub fn set_autofocus(this: &JsModalConfig, value: bool);

    /// When false, the last focused element, before the modal was shown, will not get refocused again when the modal hides. This could prevent unwanted scrolling behaviors after closing a modal.
    #[wasm_bindgen(method, setter)]
    pub fn set_restore_focus(this: &JsModalConfig, value: bool);

    /// When true, immediately shows the modal at instantiation time.
    #[wasm_bindgen(method, setter)]
    pub fn set_auto_show(this: &JsModalConfig, value: bool);

    /// Whether any change in modal DOM should automatically refresh cached positions.
    #[wasm_bindgen(method, setter)]
    pub fn set_observe_changes(this: &JsModalConfig, value: bool);

    /// If set to true will not close other visible modals when opening a new one.
    #[wasm_bindgen(method, setter)]
    pub fn set_allow_multiple(this: &JsModalConfig, value: bool);

    /// If inverted dimmer should be used.
    #[wasm_bindgen(method, setter)]
    pub fn set_inverted(this: &JsModalConfig, value: bool);

    /// If dimmer should blur background.
    #[wasm_bindgen(method, setter)]
    pub fn set_blurring(this: &JsModalConfig, value: bool);

    /// If modal should be center aligned.
    #[wasm_bindgen(method, setter)]
    pub fn set_centered(this: &JsModalConfig, value: bool);

    /// Whether to automatically bind keyboard shortcuts. This will close the modal when the ESC-Key is pressed.
    #[wasm_bindgen(method, setter)]
    pub fn set_keyboard_shortcuts(this: &JsModalConfig, value: bool);

    /// A vertical offset to allow for content outside of modal, for example a close button, to be centered.
    #[wasm_bindgen(method, setter)]
    pub fn set_offset(this: &JsModalConfig, value: u32);

    /// Selector or jquery object specifying the area to dim.
    #[wasm_bindgen(method, setter)]
    pub fn set_context(this: &JsModalConfig, value: JsValue);

    /// Setting to false will not allow you to close the modal by clicking on the dimmer.
    #[wasm_bindgen(method, setter)]
    pub fn set_closeable(this: &JsModalConfig, value: bool);

    /// You can specify custom settings to extend UI dimmer.
    #[wasm_bindgen(method, setter)]
    pub fn set_dimmer_settings(this: &JsModalConfig, value: JsValue);

    /// Named transition to use when animating menu in and out, full list can be found in ui transitions docs.
    ///
    /// Alternatively you can provide an object to set individual values for hide/show transitions as well as hide/show duration.
    ///
    /// ```
    /// {
    ///     showMethod   : 'fade',
    ///     showDuration : 200,
    ///     hideMethod   : 'zoom,
    ///     hideDuration : 500,
    /// }
    /// ```
    #[wasm_bindgen(method, setter)]
    pub fn set_transition(this: &JsModalConfig, value: JsValue);

    /// Duration of animation. The value will be ignored when individual hide/show duration values are provided via the transition setting.
    #[wasm_bindgen(method, setter)]
    pub fn set_duration(this: &JsModalConfig, value: u32);

    /// Whether additional animations should queue.
    #[wasm_bindgen(method, setter)]
    pub fn set_queue(this: &JsModalConfig, value: bool);

    /// Is called when a modal starts to show. If the function returns false, the modal will not be shown.
    #[wasm_bindgen(method, setter)]
    pub(crate) fn set_on_show(
        this: &JsModalConfig,
        value: &Closure<dyn Fn() -> bool>,
    );

    /// Is called after a modal has finished showing animating.
    #[wasm_bindgen(method, setter)]
    pub(crate) fn set_on_visible(
        this: &JsModalConfig,
        value: &Closure<dyn Fn() -> bool>,
    );

    /// Is called after a modal starts to hide. If the function returns false, the modal will not hide.
    #[wasm_bindgen(method, setter)]
    pub(crate) fn set_on_hide(
        this: &JsModalConfig,
        value: &Closure<dyn Fn(JsValue) -> bool>,
    );

    /// Is called after a modal has finished hiding animation.
    #[wasm_bindgen(method, setter)]
    pub(crate) fn set_on_hidden(
        this: &JsModalConfig,
        value: &Closure<dyn Fn() -> bool>,
    );

    /// Is called after a positive, approve or ok button is pressed. If the function returns false, the modal will not hide.
    #[wasm_bindgen(method, setter)]
    pub(crate) fn set_on_approve(
        this: &JsModalConfig,
        value: &Closure<dyn Fn(JsValue) -> bool>,
    );

    /// Is called after a negative, deny or cancel button is pressed. If the function returns false the modal will not hide.
    #[wasm_bindgen(method, setter)]
    pub(crate) fn set_on_deny(
        this: &JsModalConfig,
        value: &Closure<dyn Fn(JsValue) -> bool>,
    );

    /// Used internally to determine if the webkit custom scrollbar was clicked to prevent hiding the dimmer. This should be set to the same (numeric) value as defined for @customScrollbarWidth in site.less in case you are using a different theme.
    #[wasm_bindgen(method, setter)]
    pub fn set_scrollbar_width(this: &JsModalConfig, value: bool);

    /// Set the title.
    #[wasm_bindgen(method, setter)]
    pub fn set_title(this: &JsModalConfig, title: &str);

    /// Set the content.
    #[wasm_bindgen(method, setter)]
    pub fn set_content(this: &JsModalConfig, content: &str);

    /// Set the class.
    #[wasm_bindgen(method, setter)]
    pub fn set_class(this: &JsModalConfig, class: &str);

    /// Set wether a close icon should be shown.
    #[wasm_bindgen(method, setter)]
    pub fn set_close_icon(this: &JsModalConfig, value: bool);

    /// Set actions shown in the toast.
    #[wasm_bindgen(method, setter)]
    pub(crate) fn set_actions(
        this: &JsModalConfig,
        value: Box<[JsActionConfig]>,
    );

    /// A modal.
    pub(crate) type JsModal;

    /// Internal function to create the modal on JavaScript side.
    #[wasm_bindgen(js_namespace=["$"], js_name="modal")]
    fn new_modal(props: &JsModalConfig) -> JsModal;

    /// Internal function to create the modal alert template.
    #[wasm_bindgen(js_namespace=["$"], js_name="modal")]
    fn new_modal_alert(
        props: &str,
        title: &str,
        content: &str,
        handler: &Closure<dyn Fn()>,
    ) -> JsModal;

    /// Internal function to create the modal confirm template.
    #[wasm_bindgen(js_namespace=["$"], js_name="modal")]
    fn new_modal_confirm(
        props: &str,
        title: &str,
        content: &str,
        handler: &Closure<dyn Fn(bool)>,
    ) -> JsModal;

    /// Internal function to create the modal prompt template.
    #[wasm_bindgen(js_namespace=["$"], js_name="modal")]
    fn new_modal_prompt(
        props: &str,
        title: &str,
        content: &str,
        handler: &Closure<dyn Fn(Option<String>)>,
    ) -> JsModal;

    #[wasm_bindgen(method, js_name = "modal")]
    pub fn modal(this: &JsModal, behavior: &str);

    #[wasm_bindgen(method, js_name = "modal")]
    pub fn modal_returns_bool(this: &JsModal, behavior: &str) -> bool;

}

/*
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

    /// Internal function to create the modal alert template.
    #[wasm_bindgen(js_namespace=["$"], js_name="modal")]
    fn new_modal_alert(
        props: &JsValue,
        title: String,
        content: String,
        handler: &Closure<dyn Fn()>,
    ) -> Modal;

    /// Internal function to create the modal confirm template.
    #[wasm_bindgen(js_namespace=["$"], js_name="modal")]
    fn new_modal_confirm(
        props: &JsValue,
        title: String,
        content: String,
        handler: &Closure<dyn Fn(bool)>,
    ) -> Modal;

    /// Internal function to create the modal prompt template.
    #[wasm_bindgen(js_namespace=["$"], js_name="modal")]
    fn new_modal_prompt(
        props: &JsValue,
        title: String,
        content: String,
        handler: &Closure<dyn Fn(Option<String>)>,
    ) -> Modal;

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

    /// Creates an `Alert` modal.
    pub fn new_alert<H: 'static>(title: String, content: String, handler: H) -> anyhow::Result<Self>
    where
        H: Fn(),
    {
        let handler = Closure::new(handler);
        let result = new_modal_alert(&JsValue::from("alert"), title, content, &handler);
        handler.forget();
        Ok(result)
    }

    /// Creates a `Confirm` modal.
    pub fn new_confirm<H: 'static>(
        title: String,
        content: String,
        handler: H,
    ) -> anyhow::Result<Self>
    where
        H: Fn(bool),
    {
        let handler = Closure::new(handler);
        let result = new_modal_confirm(&JsValue::from("confirm"), title, content, &handler);
        handler.forget();
        Ok(result)
    }

    /// Creates a `Prompt` modal.
    pub fn new_prompt<H: 'static>(
        title: String,
        content: String,
        handler: H,
    ) -> anyhow::Result<Self>
    where
        H: Fn(Option<String>),
    {
        let handler = Closure::new(handler);
        let result = new_modal_prompt(&JsValue::from("prompt"), title, content, &handler);
        handler.forget();
        Ok(result)
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
*/

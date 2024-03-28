//! Toast bindings.
use crate::{
    action::JsActionConfig,
    Action,
};
use wasm_bindgen::prelude::*;

/// Display time of the [Toast].
pub enum ToastDisplayTime {
    /// The toast will be visible for the specified amount of time in milliseconds.
    Time(u32),
    /// Visible until clicked.
    UntilClicked,
    /// Time will be generated based on the amount of containing words.
    BasedOnWordAmount,
}

impl std::fmt::Display for ToastDisplayTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Time(t) => write!(f, "{t}"),
            Self::UntilClicked => write!(f, "0"),
            Self::BasedOnWordAmount => write!(f, "auto"),
        }
    }
}

/// Location of the progress bar in a toast.
#[derive(Default)]
pub enum ToastProgressBarPosition {
    /// Show it at the bottom of the toast.
    #[default]
    Bottom,
    /// Show it at the top of the toast.
    Top,
}

impl std::fmt::Display for ToastProgressBarPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bottom => write!(f, "bottom"),
            Self::Top => write!(f, "top"),
        }
    }
}

/// Toast progress bar configuration.
#[derive(Default)]
pub struct ToastProgressBar {
    /// Where the progress bar should be shown.
    pub position: ToastProgressBarPosition,
    /// CSS class name that the progress bar element will have.
    pub class: Option<String>,
    /// Wether the progress bar should be raised instead of lowered.
    pub increasing: bool,
}

/// Location of the toast.
#[derive(Default)]
pub enum ToastPosition {
    /// Show it at the bottom right of the viewport.
    #[default]
    BottomRight,
    /// Show it at the bottom left of the viewport.
    BottomLeft,
    /// Show it at the top right of the viewport.
    TopRight,
    /// Show it at the top left of the viewport.
    TopLeft,
    /// Shows the toast attached to the top of the viewport.
    TopAttached,
    /// Shows the toast attached to the bottom of the viewport.
    BottomAttached,
}

impl std::fmt::Display for ToastPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BottomRight => write!(f, "bottom right"),
            Self::BottomLeft => write!(f, "bottom left"),
            Self::TopRight => write!(f, "top right"),
            Self::TopLeft => write!(f, "top left"),
            Self::TopAttached => write!(f, "top attached"),
            Self::BottomAttached => write!(f, "bottom attached"),
        }
    }
}

/// Configuration for a [Toast] module.
pub struct ToastConfig {
    #[allow(unused)]
    handler: Closure<dyn Fn()>,
    action_handler_list: Vec<Closure<dyn Fn() -> bool>>,
    pub(crate) js_config: JsToastConfig,
}

impl ToastConfig {
    /// Creates a new [Toast] configuration.
    pub fn new() -> Self {
        let js_config = JsToastConfig::new();
        let handler = Closure::new(|| ());
        Self {
            js_config,
            handler,
            action_handler_list: vec![],
        }
    }

    /// Sets the message of the toast.
    pub fn with_message(self, message: &str) -> Self {
        self.js_config.set_message(message);
        self
    }

    /// Sets the title of the toast.
    pub fn with_title(self, title: &str) -> Self {
        self.js_config.set_title(title);
        self
    }

    /// Adds a progress bar to the toast.
    pub fn with_progress_bar(self, progress_bar: ToastProgressBar) -> Self {
        self.js_config
            .set_progress_bar_position(&progress_bar.position.to_string());
        if let Some(ref class) = progress_bar.class {
            self.js_config.set_progress_bar_class(class);
        }
        self.js_config.set_progress_up(progress_bar.increasing);
        self
    }

    /// Sets the title of the toast.
    pub fn with_class(self, class: &str) -> Self {
        self.js_config.set_class(class);
        self
    }

    /// Sets the position of the toast.
    pub fn position(self, position: ToastPosition) -> Self {
        self.js_config.set_position(&position.to_string());
        self
    }

    /// Wether the newest toast should be displayed on top.
    pub fn newest_on_top(self, is_on_top: bool) -> Self {
        self.js_config.set_newest_on_top(is_on_top);
        self
    }

    /// Wether the toast should be stacked horizontal.
    pub fn horizontal(self, horizontal: bool) -> Self {
        self.js_config.set_horizontal(horizontal);
        self
    }

    /// How long the toast should be visible.
    pub fn display_time(self, display_time: ToastDisplayTime) -> Self {
        self.js_config.set_display_time(&display_time.to_string());
        self
    }

    /// Sets the actions shown on the toast.
    pub fn with_actions(mut self, actions: Vec<Action>) -> Self {
        let mut js_actions = vec![];
        for act in actions {
            self.action_handler_list.push(act.click);
            js_actions.push(act.js_config);
        }
        self.js_config.set_actions(js_actions.into_boxed_slice());
        self
    }
}

#[wasm_bindgen]
extern "C" {
    /// The JavaScript configuration object.
    #[wasm_bindgen(js_name = Object)]
    pub(crate) type JsToastConfig;

    /// Configuration constructor.
    #[wasm_bindgen(constructor, js_class = Object)]
    pub(crate) fn new() -> JsToastConfig;

    /// Set the message to be shown.
    #[wasm_bindgen(method, setter, js_name = "message")]
    pub(crate) fn set_message(this: &JsToastConfig, message: &str);

    /// Set the title.
    #[wasm_bindgen(method, setter, js_name = "title")]
    pub(crate) fn set_title(this: &JsToastConfig, title: &str);

    /// Set the class.
    #[wasm_bindgen(method, setter, js_name = "class")]
    pub(crate) fn set_class(this: &JsToastConfig, class: &str);

    /// Set the position.
    #[wasm_bindgen(method, setter, js_name = "position")]
    pub(crate) fn set_position(this: &JsToastConfig, position: &str);

    /// Set newest on top.
    #[wasm_bindgen(method, setter, js_name = "newestOnTop")]
    pub(crate) fn set_newest_on_top(this: &JsToastConfig, is_on_top: bool);

    /// Set wether the toasts should stack horizontal.
    #[wasm_bindgen(method, setter, js_name = "horizontal")]
    pub(crate) fn set_horizontal(this: &JsToastConfig, horizontal: bool);

    /// Set how long the toast should be visible.
    #[wasm_bindgen(method, setter, js_name = "displayTime")]
    pub(crate) fn set_display_time(this: &JsToastConfig, display_time: &str);

    /// Set the progress bar position.
    #[wasm_bindgen(method, setter, js_name = "showProgress")]
    pub(crate) fn set_progress_bar_position(
        this: &JsToastConfig,
        position: &str,
    );

    /// Set the progress bar css class name.
    #[wasm_bindgen(method, setter, js_name = "classProgress")]
    pub(crate) fn set_progress_bar_class(this: &JsToastConfig, class: &str);

    /// Set the progress bar css class name.
    #[wasm_bindgen(method, setter, js_name = "progressUp")]
    pub(crate) fn set_progress_up(this: &JsToastConfig, value: bool);

    /// Set actions shown in the toast.
    #[wasm_bindgen(method, setter, js_name = "actions")]
    pub(crate) fn set_actions(
        this: &JsToastConfig,
        value: Box<[JsActionConfig]>,
    );

    /// Set the actions css class name.
    #[wasm_bindgen(method, setter, js_name = "classActions")]
    pub(crate) fn set_class_actions(this: &JsToastConfig, value: &str);

    /// Set the handler.
    #[wasm_bindgen(method, setter, js_name = "handler")]
    pub(crate) fn set_handler(
        this: &JsToastConfig,
        handler: &Closure<dyn Fn()>,
    );

    /// A toast.
    pub type Toast;

    /// Internal function to create the toast on JavaScript side.
    #[wasm_bindgen(js_namespace=["$"], js_name="toast")]
    fn new_toast(config: &JsToastConfig) -> Toast;
}

impl Toast {
    /// Creates a new [Toast].
    pub fn new(config: &ToastConfig) -> Self {
        new_toast(&config.js_config)
    }

    /// Shorthand function for a minimal [Toast] that just displays a message.
    pub fn minimal(message: &str) -> Self {
        let config = JsToastConfig::new();
        config.set_message(message);
        new_toast(&config)
    }

    /// Shorthand function for a titled [Toast] that displays a titled message.
    pub fn titled(title: &str, message: &str) -> Self {
        let config = JsToastConfig::new();
        config.set_title(title);
        config.set_message(message);
        new_toast(&config)
    }

    /// Shorthand function for a [Toast] with a message and progress bar.
    pub fn progress_bar(message: &str, progress_bar: ToastProgressBar) -> Self {
        let config = JsToastConfig::new();
        config.set_message(message);
        config.set_progress_bar_position(&progress_bar.position.to_string());
        new_toast(&config)
    }

    /// Shorthand function for a [Toast] with a message and progress bar.
    pub fn titled_progress_bar(
        title: &str,
        message: &str,
        progress_bar: ToastProgressBar,
    ) -> Self {
        let config = JsToastConfig::new();
        config.set_title(title);
        config.set_message(message);
        config.set_progress_bar_position(&progress_bar.position.to_string());
        new_toast(&config)
    }
}

/*
#[wasm_bindgen]
extern "C" {
    /// A toast configuration object.
    #[wasm_bindgen]
    pub type Object;

    /// Constructor.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Object;

    /// Set the handler.
    #[wasm_bindgen(method, setter, js_name = "title")]
    pub fn with_title(
        this: &Object,
        title: &str,
    );

    /// Set the handler.
    #[wasm_bindgen(method, setter, js_name = "handler")]
    pub fn with_handler(
        this: &Object,
        handler: &Closure<dyn Fn()>,
    );

    /// A toast.
    pub type Toast;

    /// Internal function to create the toast.
    #[wasm_bindgen(js_namespace=["$"], js_name="modal")]
    pub fn new_toast(template_name: &str, props: &Object) -> Toast;


}
    */

#![deny(missing_docs)]
//! Bindings for [fomantic-ui](https://fomantic-ui.com/).

mod action;
#[cfg(feature = "leptos")]
pub mod leptos;
#[cfg(feature = "models")]
pub mod models;
pub mod modules;

pub use action::Action;

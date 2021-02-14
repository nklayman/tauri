//! The Tauri API interface.
#![warn(missing_docs, rust_2018_idioms)]

/// The Command API module allows you to manage child processes.
pub mod command;
/// The Dialog API module allows you to show messages and prompt for file paths.
pub mod dialog;
/// The Dir module is a helper for file system directory management.
pub mod dir;
/// The File API module contains helpers to perform file operations.
pub mod file;
/// The HTTP request API.
pub mod http;
/// The file system path operations API.
pub mod path;
/// The RPC module includes utilities to send messages to the JS layer of the webview.
pub mod rpc;
/// TCP ports access API.
pub mod tcp;
/// The semver API.
pub mod version;

/// The Tauri config definition.
pub use tauri_utils::config;

/// The CLI args interface.
#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "cli")]
#[macro_use]
extern crate clap;

/// The desktop notifications API module.
#[cfg(feature = "notification")]
pub mod notification;

pub use tauri_utils::*;

mod error;

/// Tauri API error.
pub use error::Error;
/// Tauri API result type.
pub type Result<T> = std::result::Result<T, Error>;

// Not public API
#[doc(hidden)]
pub mod private {
  pub trait AsTauriContext {
    fn config_path() -> &'static std::path::Path;
    fn raw_config() -> &'static str;
    fn assets() -> &'static crate::assets::Assets;
    fn raw_tauri_script() -> &'static str;
  }
}

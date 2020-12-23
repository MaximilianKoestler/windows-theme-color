//! Simple wrapper around the winrt-API to get access to the active Windows theme colors.
//!
//! # Usage
//!
//! ```toml
//! [dependencies]
//! windows-theme-color = { git = "https://github.com/MaximilianKoestler/windows-theme-color" }
//! ```
//!
//! # Example
//!
//! A very basic example that prints the current theme colors using `dbg!(...)`
//!
//! ```rust,no_run
//! use windows_theme_color::{accent_color, background_color, foreground_color};
//!
//! fn main() {
//!     dbg!(accent_color());
//!     dbg!(background_color());
//!     dbg!(background_color());
//! }
//! ```

mod base;
pub use base::*;

#[cfg(all(feature = "piet"))]
pub mod piet;

#[cfg(all(feature = "piet"))]
pub use piet::*;

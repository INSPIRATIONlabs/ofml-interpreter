//! TUI Views
//!
//! This module contains individual view renderers for each screen.

#[cfg(feature = "tui")]
pub mod articles;
#[cfg(feature = "tui")]
pub mod catalog;
#[cfg(feature = "tui")]
pub mod families;
#[cfg(feature = "tui")]
pub mod family_config;
#[cfg(feature = "tui")]
pub mod help;
#[cfg(feature = "tui")]
pub mod manufacturers;
#[cfg(feature = "tui")]
pub mod properties;
#[cfg(feature = "tui")]
pub mod tables;

//! TUI (Terminal User Interface) for OAP Configurator
//!
//! This module provides an interactive terminal interface for:
//! - Browsing manufacturers and articles
//! - Configuring product properties
//! - Viewing prices in real-time
//! - Exporting configurations
//!
//! Built with ratatui and crossterm following the Elm Architecture (TEA) pattern.

#[cfg(feature = "tui")]
pub mod app;
#[cfg(feature = "tui")]
pub mod ui;
#[cfg(feature = "tui")]
pub mod views;
#[cfg(feature = "tui")]
pub mod widgets;

#[cfg(feature = "tui")]
pub use app::{App, Message, Screen};

//! TUI (Terminal User Interface) for OAP Configurator
//!
//! This crate provides an interactive terminal interface for:
//! - Browsing manufacturers and articles
//! - Configuring product properties
//! - Viewing prices in real-time
//! - Exporting configurations
//!
//! Built with ratatui and crossterm following the Elm Architecture (TEA) pattern.

pub mod app;
pub mod config_store;
pub mod theme;
pub mod ui;
pub mod views;
pub mod widgets;

pub use app::{App, Message, Screen};
pub use config_store::SavedConfiguration;
pub use theme::{Theme, ThemeVariant};

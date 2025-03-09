// reader/mod.rs
pub mod ui;

// Re-export the main reader functions for backwards compatibility
pub use ui::{start_capture, set_inventory_ui};
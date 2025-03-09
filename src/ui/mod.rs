// ui/mod.rs
pub mod converter;
pub mod common;

// Re-export the primary UI functions
pub use common::{
    create_reader_tab,
    create_conversion_tab,
    create_batch_tab
};

// Additional UI helpers
pub fn init_ui() {
    // Set application-wide UI settings if needed
    fltk::app::set_visible_focus(false);
    fltk::app::set_scrollbar_size(15);
}
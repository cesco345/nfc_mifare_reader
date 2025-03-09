// batch/mod.rs
pub mod ui;

// Re-export primary functions for convenience
pub use crate::batch::process_batch as batch_process;

// Handle re-exporting batch.rs functionality for backward compatibility
use std::cell::RefCell;
use std::rc::Rc;
use fltk::text::TextBuffer;

pub fn process_batch(
    text: &str,
    kb_layout: i32,
    result_buffer: Rc<RefCell<TextBuffer>>
) {
    crate::batch::ui::process_batch(text, kb_layout, result_buffer)
}
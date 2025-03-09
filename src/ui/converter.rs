// ui/converter.rs
use std::cell::RefCell;
use std::rc::Rc;
use fltk::text::TextBuffer;

// Import utility functions from the utils module
use crate::utils;

pub fn convert_uid(
    uid: &str,
    keyboard_layout: i32,
    hex_buffer: Rc<RefCell<TextBuffer>>,
    dec_buffer: Rc<RefCell<TextBuffer>>,
    mfg_buffer: Rc<RefCell<TextBuffer>>,
    format_buffer: Rc<RefCell<TextBuffer>>
) {
    if uid.is_empty() {
        // Clear all buffers if input is empty
        hex_buffer.borrow_mut().set_text("");
        dec_buffer.borrow_mut().set_text("");
        mfg_buffer.borrow_mut().set_text("");
        format_buffer.borrow_mut().set_text("");
        return;
    }
    
    // Process the UID with the selected keyboard layout
    let (hex_uid, manufacturer) = utils::process_uid_for_display(uid, keyboard_layout);
    
    // Calculate decimal value
    let decimal_value = utils::hex_to_decimal(&hex_uid);
    
    // Determine format
    let format_desc = utils::interpret_format_code(uid);
    
    // Update display buffers
    hex_buffer.borrow_mut().set_text(&hex_uid);
    dec_buffer.borrow_mut().set_text(&decimal_value);
    mfg_buffer.borrow_mut().set_text(&manufacturer);
    format_buffer.borrow_mut().set_text(&format_desc);
}
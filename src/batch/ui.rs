// /batch/ui.rs
use std::cell::RefCell;
use std::rc::Rc;
use fltk::text::TextBuffer;

use crate::utils;

pub fn process_batch(
    text: &str,
    kb_layout: i32,
    result_buffer: Rc<RefCell<TextBuffer>>
) {
    let lines: Vec<&str> = text.split('\n').collect();
    
    let mut results = String::new();
    
    for (i, line) in lines.iter().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        
        let (hex_uid, manufacturer) = utils::process_uid_for_display(line, kb_layout);
        let format_desc = utils::interpret_format_code(line);
        
        // Calculate decimal value
        let decimal_value = utils::hex_to_decimal(&hex_uid);
        
        results.push_str(&format!("UID #{}: {}\n", i + 1, line));
        results.push_str(&format!("   → Hex: {}\n", hex_uid));
        results.push_str(&format!("   → Decimal: {}\n", decimal_value));
        results.push_str(&format!("   → Manufacturer: {}\n", manufacturer));
        results.push_str(&format!("   → Format: {}\n\n", format_desc));
    }
    
    result_buffer.borrow_mut().set_text(&results);
}
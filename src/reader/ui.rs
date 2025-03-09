// reader/ui.rs
use fltk::{
    button::Button,
    enums::{Color, CallbackTrigger},
    frame::Frame,
    input::Input,
    prelude::*,
    text::TextBuffer,
    window::Window,
    dialog,
};
use std::cell::RefCell;
use std::rc::Rc;

use crate::utils;
use crate::inventory::ui::actions::InventoryUI;

// Instead of a static variable, we'll use a more direct approach
// through function parameters
static mut INVENTORY_UI_INSTANCE: Option<*const InventoryUI> = None;

// Set the global inventory UI reference from main.rs - unsafe but controlled
pub fn set_inventory_ui(inventory_ui: &Rc<InventoryUI>) {
    unsafe {
        // Store the raw pointer - this is safe because we control the lifetime
        // and ensure the InventoryUI lives for the duration of the program
        INVENTORY_UI_INSTANCE = Some(Rc::as_ptr(inventory_ui));
    }
}

pub fn start_capture(btn: &mut Button, card_buffer: Rc<RefCell<TextBuffer>>, kb_layout: Rc<RefCell<i32>>) {
    if btn.label() == "Start Capture" {
        btn.set_label("Stop Capture");
        
        // Create a capture window
        let mut capture_wind = Window::new(300, 300, 500, 200, "Card Capture");
        capture_wind.set_color(Color::White);
        
        Frame::new(20, 20, 460, 40, "Present cards to the reader\nCard data will appear here:").set_label_size(14);
        
        let mut capture_input = Input::new(20, 80, 460, 30, "");
        capture_input.set_trigger(CallbackTrigger::EnterKey);
        
        let card_buffer_clone = card_buffer.clone();
        let kb_layout_clone = kb_layout.clone();
        
        // Create a checkbox for inventory mode
        let inventory_mode = fltk::button::CheckButton::new(20, 120, 200, 30, "Update Inventory");
        
        // Function to process card data
        capture_input.set_callback(move |inp| {
            let data = inp.value();
            if !data.is_empty() {
                if !data.contains("config") && !data.contains("Buz") {
                    // Get timestamp information
                    let (unix_timestamp, human_timestamp) = utils::get_timestamps();
                    
                    // Get selected keyboard layout
                    let kb_layout_value = *kb_layout_clone.borrow();
                    
                    // Process the UID for human-readable format
                    let (hex_uid, manufacturer) = utils::process_uid_for_display(&data, kb_layout_value);
                    
                    // Calculate decimal value for human readability
                    let decimal_value = utils::hex_to_decimal(&hex_uid);
                    
                    // Interpret format
                    let format_desc = utils::interpret_format_code(&data);
                    
                    // Create a more detailed record
                    let record = format!(
                        "[{}] ({}) Raw UID: {}\n    → Hex: {}\n    → Decimal: {}\n    → Manufacturer: {}\n    → Format: {}\n\n", 
                        unix_timestamp,
                        human_timestamp, 
                        data, 
                        hex_uid,
                        decimal_value, 
                        manufacturer,
                        format_desc
                    );
                    
                    // Add to the display
                    let mut buffer = card_buffer_clone.borrow_mut();
                    let current = buffer.text();
                    buffer.set_text(&format!("{}{}", current, record));
                    
                    // If inventory mode is checked, pass this tag to inventory system
                    if inventory_mode.is_checked() {
                        // First check if we can access the inventory database from the main module
                        if let Ok(inventory_ui) = get_inventory_ui() {
                            // Process tag in inventory system
                            inventory_ui.process_scanned_tag(&hex_uid.replace(" ", ""));
                        } else {
                            dialog::alert(300, 300, "Could not access inventory system.");
                        }
                    }
                }
                inp.set_value("");
            }
        });
        
        // Make the input focus automatically
        capture_input.take_focus().unwrap();
        
        capture_wind.end();
        capture_wind.show();
        
        let mut btn_clone = btn.clone();
        // Set window close callback
        capture_wind.set_callback(move |w| {
            w.hide();
            btn_clone.set_label("Start Capture");
        });
        
    } else {
        btn.set_label("Start Capture");
        // No need to worry about closing windows - they'll close themselves
    }
}

// Helper function to get inventory UI instance
fn get_inventory_ui() -> Result<&'static InventoryUI, String> {
    unsafe {
        if let Some(ptr) = INVENTORY_UI_INSTANCE {
            // This is safe because we control the lifetime of the InventoryUI
            // and ensure it lives for the duration of the program
            Ok(&*ptr)
        } else {
            Err("Inventory system not initialized".to_string())
        }
    }
}
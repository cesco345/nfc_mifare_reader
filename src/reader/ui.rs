// reader/ui.rs
use fltk::{
    button::Button,
    enums::{Color, CallbackTrigger, Font},
    frame::Frame,
    input::{Input, MultilineInput},
    prelude::*,
    text::TextBuffer,
    window::Window,
    dialog,
    menu::Choice,
    group::Group,
};
use std::cell::RefCell;
use std::rc::Rc;

use crate::utils;
use crate::inventory::ui::actions::InventoryUI;
use crate::inventory::model::{create_inventory_item, generate_timestamp, InventoryItem};

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
        inventory_mode.set_checked(true); // Enable by default

        // Create a checkbox for showing item form when scanning
        let show_form = fltk::button::CheckButton::new(220, 120, 260, 30, "Show Item Form When Scanning");
        show_form.set_checked(true); // Enable by default
        
        // Function to process card data
        let show_form_clone = show_form.clone();
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
                    
                    // Clean the tag ID (remove spaces)
                    let clean_tag_id = hex_uid.replace(" ", "");
                    
                    // If inventory mode is checked, pass this tag to inventory system
                    if inventory_mode.is_checked() {
                        // First check if we can access the inventory database from the main module
                        if let Ok(inventory_ui) = get_inventory_ui() {
                            // Check if the tag already exists in inventory
                            match inventory_ui.inventory_db.borrow().get_item(&clean_tag_id) {
                                Ok(Some(item)) => {
                                    // Item exists - show quick update dialog if form is enabled
                                    if show_form_clone.is_checked() {
                                        show_item_update_dialog(inventory_ui, item.clone());
                                    } else {
                                        // Just increment quantity by default
                                        if let Err(e) = inventory_ui.inventory_db.borrow().update_quantity(&clean_tag_id, item.quantity + 1) {
                                            dialog::alert(300, 300, &format!("Error updating quantity: {}", e));
                                        } else {
                                            dialog::message(300, 300, &format!("Updated quantity of '{}' to {}", item.name, item.quantity + 1));
                                        }
                                    }
                                },
                                Ok(None) => {
                                    // New item - show creation dialog if form is enabled
                                    if show_form_clone.is_checked() {
                                        show_new_item_dialog(inventory_ui, clean_tag_id.clone(), manufacturer.clone());
                                    } else {
                                        // Ask to create a simple item
                                        if dialog::choice2(300, 300, &format!("Tag ID {} not found in inventory. Create a new item?", clean_tag_id), "No", "Yes", "") == Some(1) {
                                            if let Some(name) = dialog::input(300, 300, "Enter item name:", "") {
                                                if !name.is_empty() {
                                                    let new_item = create_inventory_item(
                                                        &clean_tag_id,
                                                        &name,
                                                        None,
                                                        1,
                                                        None,
                                                        None
                                                    );
                                                    
                                                    if let Err(e) = inventory_ui.inventory_db.borrow().save_item(&new_item) {
                                                        dialog::alert(300, 300, &format!("Error saving item: {}", e));
                                                    } else {
                                                        dialog::message(300, 300, &format!("New item '{}' added to inventory", name));
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                                Err(e) => {
                                    dialog::alert(300, 300, &format!("Error checking inventory: {}", e));
                                }
                            }
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

// New function to show item creation dialog - Note: takes ownership of tag_id and manufacturer
fn show_new_item_dialog(inventory_ui: &'static InventoryUI, tag_id: String, manufacturer: String) {
    // Create modal window
    let mut win = Window::new(300, 200, 450, 450, "New Item");
    win.make_modal(true);
    
    // Add title
    let mut title = Frame::new(0, 10, 450, 30, "Add New Inventory Item");
    title.set_label_font(Font::HelveticaBold);
    title.set_label_size(18);
    
    // Tag ID display
    let tag_label = format!("Tag ID: {}", tag_id);
    let mut tag_frame = Frame::new(20, 50, 410, 30, tag_label.as_str());
    tag_frame.set_label_font(Font::HelveticaBold);
    
    // Manufacturer display
    let manuf_label = format!("Manufacturer: {}", manufacturer);
    Frame::new(20, 80, 410, 30, manuf_label.as_str());
    
    // Add form elements with labels
    let mut name_input = Input::new(150, 120, 270, 30, "Name:");
    let mut desc_input = MultilineInput::new(150, 160, 270, 70, "Description:");
    let mut qty_input = Input::new(150, 240, 270, 30, "Quantity:");
    qty_input.set_value("1"); // Default quantity
    
    let mut location_input = Input::new(150, 280, 270, 30, "Location:");
    
    let mut category_choice = Choice::new(150, 320, 270, 30, "Category:");
    // Get categories from database and populate the dropdown
    if let Ok(categories_with_count) = inventory_ui.inventory_db.borrow().get_categories() {
        category_choice.add_choice("Uncategorized");
        for (category, _) in categories_with_count {
            category_choice.add_choice(&category);
        }
    }
    
    // Add save and cancel buttons
    let mut save_btn = Button::new(120, 380, 100, 40, "Save");
    let mut cancel_btn = Button::new(230, 380, 100, 40, "Cancel");
    
    win.end();
    win.show();
    
    // Setup save button callback - Clone what we need to access inside callback
    let tag_id_for_save = tag_id.clone();
    let mut win_copy = win.clone();
    let name_input_clone = name_input.clone();
    let desc_input_clone = desc_input.clone();
    let qty_input_clone = qty_input.clone();
    let location_input_clone = location_input.clone();
    let category_choice_clone = category_choice.clone();
    
    save_btn.set_callback(move |_| {
        // Validate inputs
        if name_input_clone.value().is_empty() {
            dialog::alert(300, 300, "Item name is required");
            return;
        }
        
        // Create the item with proper String handling
        let qty = qty_input_clone.value().parse::<i32>().unwrap_or(1);
        
        let category = if category_choice_clone.value() > 0 {
            category_choice_clone.text(category_choice_clone.value())
        } else {
            None
        };
        
        let description = if desc_input_clone.value().is_empty() { 
            None 
        } else { 
            Some(desc_input_clone.value()) 
        };
        
        let location = if location_input_clone.value().is_empty() { 
            None 
        } else { 
            Some(location_input_clone.value()) 
        };
        
        let new_item = create_inventory_item(
            &tag_id_for_save,
            &name_input_clone.value(),
            description.as_deref(),
            qty,
            location.as_deref(),
            category.as_deref()
        );
        
        // Save to database
        if let Err(e) = inventory_ui.inventory_db.borrow().save_item(&new_item) {
            dialog::alert(300, 300, &format!("Error saving item: {}", e));
        } else {
            dialog::message(300, 300, &format!("New item '{}' added to inventory", name_input_clone.value()));
            win_copy.hide();
        }
    });
    
    // Setup cancel button callback
    cancel_btn.set_callback(move |_| {
        win.hide();
    });
}

// New function to show item update dialog - Note: takes ownership of the item
fn show_item_update_dialog(inventory_ui: &'static InventoryUI, item: InventoryItem) {
    // Create modal window
    let mut win = Window::new(300, 200, 450, 500, "Update Item");
    win.make_modal(true);
    
    // Add title
    let mut title = Frame::new(0, 10, 450, 30, "Update Inventory Item");
    title.set_label_font(Font::HelveticaBold);
    title.set_label_size(18);
    
    // Item information display
    let info_text = format!(
        "Item: {}\nTag ID: {}", 
        item.name, 
        item.tag_id
    );
    let mut info_frame = Frame::new(0, 40, 450, 60, info_text.as_str());
    info_frame.set_label_font(Font::HelveticaBold);
    
    // Create update form
    let form_group = Group::new(20, 110, 410, 300, "");
    
    // Current quantity display
    let qty_text = format!("Current Quantity: {}", item.quantity);
    Frame::new(20, 110, 410, 30, qty_text.as_str());
    
    // Quick quantity update controls
    let mut decrement_btn = Button::new(120, 150, 40, 40, "-");
    let mut increment_btn = Button::new(290, 150, 40, 40, "+");
    let mut new_qty_input = Input::new(170, 155, 110, 30, "");
    new_qty_input.set_value(&item.quantity.to_string());
    
    // Location update
    Frame::new(20, 200, 100, 30, "Location:");
    let mut location_input = Input::new(120, 200, 310, 30, "");
    location_input.set_value(&item.location.clone().unwrap_or_default());
    
    // Category update
    Frame::new(20, 240, 100, 30, "Category:");
    let mut category_choice = Choice::new(120, 240, 310, 30, "");
    
    // Populate categories dropdown
    if let Ok(categories_with_count) = inventory_ui.inventory_db.borrow().get_categories() {
        category_choice.add_choice("Uncategorized");
        let mut selected_index = 0;
        
        for (i, (category, _)) in categories_with_count.iter().enumerate() {
            category_choice.add_choice(category);
            if let Some(ref item_category) = item.category {
                if item_category == category {
                    selected_index = i + 1; // +1 because Uncategorized is at index 0
                }
            }
        }
        
        category_choice.set_value(selected_index as i32);
    }
    
    // Description update
    Frame::new(20, 280, 410, 20, "Description:");
    let mut desc_input = MultilineInput::new(20, 300, 410, 80, "");
    desc_input.set_value(&item.description.clone().unwrap_or_default());
    
    form_group.end();
    
    // Add save, delete, and cancel buttons
    let mut save_btn = Button::new(90, 400, 90, 40, "Save");
    let mut delete_btn = Button::new(190, 400, 90, 40, "Delete");
    let mut cancel_btn = Button::new(290, 400, 90, 40, "Cancel");
    
    win.end();
    win.show();
    
    // Setup increment/decrement callbacks with mutable clones
    let mut new_qty_input_dec = new_qty_input.clone();
    decrement_btn.set_callback(move |_| {
        let current = new_qty_input_dec.value().parse::<i32>().unwrap_or(0);
        if current > 0 {
            new_qty_input_dec.set_value(&(current - 1).to_string());
        }
    });
    
    let mut new_qty_input_inc = new_qty_input.clone();
    increment_btn.set_callback(move |_| {
        let current = new_qty_input_inc.value().parse::<i32>().unwrap_or(0);
        new_qty_input_inc.set_value(&(current + 1).to_string());
    });
    
    // Setup save button callback with proper clones
    let mut win_copy = win.clone();
    // Save a separate copy of tag_id for the save button
    let tag_id_for_save = item.tag_id.clone();
    let name = item.name.clone();
    let created_at = item.created_at.clone();
    
    let new_qty_input_save = new_qty_input.clone();
    let location_input_save = location_input.clone();
    let category_choice_save = category_choice.clone();
    let desc_input_save = desc_input.clone();
    
    save_btn.set_callback(move |_| {
        // Get values from form
        let new_qty = new_qty_input_save.value().parse::<i32>().unwrap_or(item.quantity);
        
        // Create a new item instead of trying to modify the referenced one
        let mut updated_item = InventoryItem {
            tag_id: tag_id_for_save.clone(),
            name: name.clone(),
            description: None,
            quantity: new_qty,
            location: None,
            category: None,
            last_updated: generate_timestamp(),
            created_at: created_at.clone(),
        };
        
        // Set optional fields
        updated_item.location = if location_input_save.value().is_empty() { 
            None 
        } else { 
            Some(location_input_save.value()) 
        };
        
        updated_item.category = if category_choice_save.value() <= 0 {
            None
        } else if let Some(cat_text) = category_choice_save.text(category_choice_save.value()) {
            Some(cat_text)
        } else {
            item.category.clone()
        };
        
        updated_item.description = if desc_input_save.value().is_empty() {
            None
        } else {
            Some(desc_input_save.value())
        };
        
        // Save to database
        if let Err(e) = inventory_ui.inventory_db.borrow().save_item(&updated_item) {
            dialog::alert(300, 300, &format!("Error updating item: {}", e));
        } else {
            dialog::message(300, 300, &format!("Item '{}' updated", name));
            win_copy.hide();
        }
    });
    
    // Setup delete button callback with a separate tag_id clone
    let mut win_delete = win.clone();
    let delete_tag_id = item.tag_id.clone();
    delete_btn.set_callback(move |_| {
        if dialog::choice2(300, 300, "Are you sure you want to delete this item?", "No", "Yes", "") == Some(1) {
            // Delete from database
            if let Err(e) = inventory_ui.inventory_db.borrow().delete_item(&delete_tag_id) {
                dialog::alert(300, 300, &format!("Error deleting item: {}", e));
            } else {
                dialog::message(300, 300, "Item deleted successfully");
                win_delete.hide();
            }
        }
    });
    
    // Setup cancel button callback
    cancel_btn.set_callback(move |_| {
        win.hide();
    });
}
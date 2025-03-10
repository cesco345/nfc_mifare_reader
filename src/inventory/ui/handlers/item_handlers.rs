// src/inventory/ui/handlers/item_handlers.rs
use fltk::{
    button::Button,
    dialog,
    prelude::*,
    text::TextBuffer,
    table::Table,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashSet;

use crate::inventory::model::InventoryItem;
use crate::inventory::db::InventoryDB;
use crate::inventory::ui::components::form::ItemForm;
use crate::inventory::ui::utils::ChoiceExt;

pub fn setup_save_button(
    save_btn: &mut Button,
    item_form: &ItemForm,
    log_buffer: &TextBuffer,
    inventory_db: Rc<RefCell<InventoryDB>>,
    items: Rc<RefCell<Vec<InventoryItem>>>,
    current_tag_id: Rc<RefCell<Option<String>>>,
    item_table: Rc<RefCell<Table>>
) {
    let db_clone = inventory_db;
    let items_clone = items;
    let current_tag_clone = current_tag_id;
    let table_clone = item_table;
    let mut log_buffer_clone = log_buffer.clone();
    let item_form_clone = item_form.clone(); // Clone here to use in callback
    
    save_btn.set_callback(move |_| {
        if let Some(tag_id) = current_tag_clone.borrow().clone() {
            match item_form_clone.get_form_data(&tag_id) {
                Ok(mut item) => {
                    // Get created_at date from existing item if possible
                    if let Ok(Some(existing_item)) = db_clone.borrow().get_item(&tag_id) {
                        // Keep the original creation date
                        item.created_at = existing_item.created_at.clone();
                    }
                    
                    // Save to database
                    if let Err(e) = db_clone.borrow().save_item(&item) {
                        dialog::alert(300, 300, &format!("Error saving item: {}", e));
                        return;
                    }
                    
                    // Update local items list and table
                    if let Ok(all_items) = db_clone.borrow().get_all_items() {
                        *items_clone.borrow_mut() = all_items;
                        table_clone.borrow_mut().set_rows(items_clone.borrow().len() as i32);
                        table_clone.borrow_mut().redraw();
                    }
                    
                    log_buffer_clone.append(&format!("Saved item: {}\n", item.name));
                    dialog::message(300, 300, "Item saved successfully");
                },
                Err(e) => {
                    dialog::alert(300, 300, &format!("Form validation error: {}", e));
                }
            }
        } else {
            dialog::alert(300, 300, "No item selected to save");
        }
    });
}

pub fn setup_delete_button(
    delete_btn: &mut Button,
    item_form: &mut ItemForm,
    log_buffer: &TextBuffer,
    inventory_db: Rc<RefCell<InventoryDB>>,
    items: Rc<RefCell<Vec<InventoryItem>>>,
    current_tag_id: Rc<RefCell<Option<String>>>,
    item_table: Rc<RefCell<Table>>
) {
    let db_clone = inventory_db;
    let items_clone = items;
    let current_tag_clone = current_tag_id;
    let table_clone = item_table;
    let mut log_buffer_clone = log_buffer.clone();
    let mut item_form_clone = item_form.clone();
    
    delete_btn.set_callback(move |_| {
        if let Some(tag_id) = current_tag_clone.borrow().clone() {
            // Ask for confirmation
            if dialog::choice2(300, 300, "Are you sure you want to delete this item?", "No", "Yes", "") == Some(1) {
                // Delete from database
                if let Err(e) = db_clone.borrow().delete_item(&tag_id) {
                    dialog::alert(300, 300, &format!("Error deleting item: {}", e));
                    return;
                }
                
                // Clear form
                item_form_clone.clear();
                
                // Clear selected tag
                *current_tag_clone.borrow_mut() = None;
                
                // Update local items list and table
                if let Ok(all_items) = db_clone.borrow().get_all_items() {
                    *items_clone.borrow_mut() = all_items;
                    table_clone.borrow_mut().set_rows(items_clone.borrow().len() as i32);
                    table_clone.borrow_mut().redraw();
                }
                
                log_buffer_clone.append(&format!("Deleted item with tag: {}\n", tag_id));
                dialog::message(300, 300, "Item deleted successfully");
            }
        } else {
            dialog::alert(300, 300, "No item selected to delete");
        }
    });
}

pub fn setup_clear_button(
    clear_btn: &mut Button,
    item_form: &mut ItemForm,
    log_buffer: &TextBuffer,
    current_tag_id: Rc<RefCell<Option<String>>>
) {
    let current_tag_clone = current_tag_id;
    let mut log_buffer_clone = log_buffer.clone();
    let mut item_form_clone = item_form.clone();
    
    clear_btn.set_callback(move |_| {
        item_form_clone.clear();
        *current_tag_clone.borrow_mut() = None;
        log_buffer_clone.append("Form cleared\n");
    });
}

pub fn setup_add_button(
    add_btn: &mut Button,
    item_form: &mut ItemForm,
    log_buffer: &TextBuffer,
    current_tag_id: Rc<RefCell<Option<String>>>
) {
    let current_tag_clone = current_tag_id;
    let mut log_buffer_clone = log_buffer.clone();
    let mut item_form_clone = item_form.clone();
    
    add_btn.set_callback(move |_| {
        // Generate a new tag ID or prompt user for one
        if let Some(tag_id) = dialog::input(300, 300, "Enter Tag ID for new item:", "") {
            if !tag_id.is_empty() {
                // Clear form and set new tag ID
                item_form_clone.clear();
                
                // Clone tag_id before moving it
                let display_tag_id = tag_id.clone();
                let log_tag_id = tag_id.clone();
                
                item_form_clone.tag_id_display.set_label(&format!("Tag ID: {}", display_tag_id));
                *current_tag_clone.borrow_mut() = Some(tag_id);
                log_buffer_clone.append(&format!("Ready to add new item with tag: {}\n", log_tag_id));
            }
        }
    });
}

pub fn setup_refresh_button(
    refresh_btn: &mut Button,
    stats_text: &mut fltk::frame::Frame,
    category_choice: &mut fltk::menu::Choice,
    log_buffer: &TextBuffer,
    inventory_db: Rc<RefCell<InventoryDB>>,
    items: Rc<RefCell<Vec<InventoryItem>>>,
    item_table: Rc<RefCell<Table>>
) {
    let db_clone = inventory_db;
    let items_clone = items;
    let table_clone = item_table;
    let mut stats_text_clone = stats_text.clone();
    let mut category_choice_clone = category_choice.clone();
    let mut log_buffer_clone = log_buffer.clone();
    
    refresh_btn.set_callback(move |_| {
        match db_clone.borrow().get_all_items() {
            Ok(all_items) => {
                // Update items list
                *items_clone.borrow_mut() = all_items;
                
                // Update table
                let items = items_clone.borrow();
                let mut table = table_clone.borrow_mut();
                table.set_rows(items.len() as i32);
                
                // Update stats
                let total_quantity: i32 = items.iter().map(|i| i.quantity).sum();
                let categories: HashSet<_> = items
                    .iter()
                    .filter_map(|i| i.category.clone())
                    .collect();
                
                stats_text_clone.set_label(&format!(
                    "Total Items: {}\nTotal Quantity: {}\nCategories: {}",
                    items.len(),
                    total_quantity,
                    categories.len()
                ));
                
                // Populate category dropdown
                let mut categories: Vec<String> = categories.into_iter().collect();
                categories.sort();
                category_choice_clone.update_categories(&categories);
                
                // Add to log
                log_buffer_clone.append("Refreshed inventory list\n");
            },
            Err(e) => {
                dialog::alert(300, 300, &format!("Error loading inventory: {}", e));
                log_buffer_clone.append(&format!("Error: {}\n", e));
            }
        }
    });
}
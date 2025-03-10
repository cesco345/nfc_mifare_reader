// src/inventory/ui/handlers/scan_handlers.rs
use fltk::{
    dialog,
    prelude::*,
    table::Table,
};
use std::cell::RefCell;
use std::rc::Rc;

use crate::inventory::db::InventoryDB;
use crate::inventory::model::{InventoryItem, create_inventory_item};

pub fn process_scanned_tag(
    tag_id: &str,
    inventory_db: &Rc<RefCell<InventoryDB>>,
    current_tag_id: &Rc<RefCell<Option<String>>>,
    items: &Rc<RefCell<Vec<InventoryItem>>>,
    item_table: &Rc<RefCell<Table>>
) {
    // Check if tag exists in inventory
    match inventory_db.borrow().get_item(tag_id) {
        Ok(Some(item)) => {
            // Item exists - increment quantity
            let new_quantity = item.quantity + 1;
            if let Err(e) = inventory_db.borrow().update_quantity(tag_id, new_quantity) {
                dialog::alert(300, 300, &format!("Error updating quantity: {}", e));
                return;
            }
            
            dialog::message(300, 300, &format!("Tag scanned: {}. Quantity updated to {}.", item.name, new_quantity));
        },
        Ok(None) => {
            // Item doesn't exist - ask to create
            if dialog::choice2(300, 300, 
                &format!("Tag ID {} not found in inventory. Would you like to add a new item?", tag_id),
                "No", "Yes", "") == Some(1) {
                
                // Set current tag and prompt for details
                *current_tag_id.borrow_mut() = Some(tag_id.to_string());
                
                // This would ideally open a form dialog, but for now we'll use a simple input
                if let Some(name) = dialog::input(300, 300, "Enter item name:", "") {
                    if !name.is_empty() {
                        // Create basic item
                        let item = create_inventory_item(tag_id, &name, None, 1, None, None);
                        
                        // Save to database
                        if let Err(e) = inventory_db.borrow().save_item(&item) {
                            dialog::alert(300, 300, &format!("Error saving item: {}", e));
                            return;
                        }
                        
                        // Add Google Drive sync if enabled
                        sync_to_gdrive(inventory_db);
                        
                        dialog::message(300, 300, &format!("New item '{}' added to inventory.", name));
                        
                        // Refresh the table
                        if let Ok(all_items) = inventory_db.borrow().get_all_items() {
                            *items.borrow_mut() = all_items;
                            let mut table = item_table.borrow_mut();
                            table.set_rows(items.borrow().len() as i32);
                            table.redraw();
                        }
                    }
                }
            }
        },
        Err(e) => {
            dialog::alert(300, 300, &format!("Error checking inventory: {}", e));
        }
    }
}

fn sync_to_gdrive(inventory_db: &Rc<RefCell<InventoryDB>>) {
    // Add Google Drive sync if enabled
    // Update APP_CONFIG access depending on your final solution
    // For Mutex-based approach:
    if let Ok(config) = crate::config::APP_CONFIG.lock() {
        if config.gdrive_sync_enabled {
            use crate::sync::gdrive_sync::GDriveSync;
            let gdrive_sync = GDriveSync::new(&config.gdrive_sync_folder);
            match gdrive_sync.export_database(&inventory_db.borrow()) {
                Ok(_) => println!("Automatically synced database to Google Drive"),
                Err(e) => println!("Error auto-syncing to Google Drive: {}", e)
            }
        }
    }
}
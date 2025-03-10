// src/inventory/ui/handlers/export_handlers.rs
use fltk::{
    button::Button,
    dialog,
    prelude::*,
    text::TextBuffer,
};
use std::cell::RefCell;
use std::rc::Rc;

use crate::inventory::db::InventoryDB;

pub fn setup_export_button(
    export_btn: &mut Button,
    log_buffer: &TextBuffer,
    inventory_db: Rc<RefCell<InventoryDB>>
) {
    let db_clone = inventory_db;
    let mut log_buffer_clone = log_buffer.clone();
    
    export_btn.set_callback(move |_| {
        // Fixed the dialog::choice call to use dialog::choice2
        match dialog::choice2(300, 300, "Select export format:", "JSON", "CSV", "Cancel") {
            Some(1) => { // JSON
                if let Some(path) = dialog::file_chooser("Save JSON Export", "*.json", "", false) {
                    match db_clone.borrow().export_json() {
                        Ok(json) => {
                            if let Err(e) = std::fs::write(&path, json) {
                                dialog::alert(300, 300, &format!("Error writing file: {}", e));
                            } else {
                                log_buffer_clone.append(&format!("Exported JSON to {}\n", path));
                                dialog::message(300, 300, &format!("Data exported to {}", path));
                            }
                        },
                        Err(e) => dialog::alert(300, 300, &format!("Error exporting data: {}", e))
                    }
                }
            },
            Some(2) => { // CSV
                if let Some(path) = dialog::file_chooser("Save CSV Export", "*.csv", "", false) {
                    match db_clone.borrow().export_csv() {
                        Ok(csv) => {
                            if let Err(e) = std::fs::write(&path, csv) {
                                dialog::alert(300, 300, &format!("Error writing file: {}", e));
                            } else {
                                log_buffer_clone.append(&format!("Exported CSV to {}\n", path));
                                dialog::message(300, 300, &format!("Data exported to {}", path));
                            }
                        },
                        Err(e) => dialog::alert(300, 300, &format!("Error exporting data: {}", e))
                    }
                }
            },
            _ => {} // Cancel or no choice
        }
    });
}

pub fn setup_import_button(
    import_btn: &mut Button,
    log_buffer: &TextBuffer,
    inventory_db: Rc<RefCell<InventoryDB>>,
    refresh_callback: impl Fn() + 'static
) {
    let db_clone = inventory_db;
    let mut log_buffer_clone = log_buffer.clone();
    
    import_btn.set_callback(move |_| {
        match dialog::choice2(300, 300, "Select import format:", "JSON", "CSV", "Cancel") {
            Some(1) => { // JSON
                if let Some(path) = dialog::file_chooser("Open JSON Import", "*.json", "", true) {
                    match std::fs::read_to_string(&path) {
                        Ok(json) => {
                            match db_clone.borrow().import_json(&json) {
                                Ok(count) => {
                                    log_buffer_clone.append(&format!("Imported {} items from {}\n", count, path));
                                    dialog::message(300, 300, &format!("Successfully imported {} items", count));
                                    refresh_callback();
                                },
                                Err(e) => dialog::alert(300, 300, &format!("Error importing data: {}", e))
                            }
                        },
                        Err(e) => dialog::alert(300, 300, &format!("Error reading file: {}", e))
                    }
                }
            },
            // CSV import would be implemented here
            _ => {} // Cancel or no choice
        }
    });
}
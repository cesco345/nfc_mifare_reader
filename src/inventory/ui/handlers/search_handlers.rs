// src/inventory/ui/handlers/search_handlers.rs
use fltk::{
    button::Button,
    dialog,
    input::Input,
    prelude::*,
    text::TextBuffer,
    table::Table,
};
use std::cell::RefCell;
use std::rc::Rc;

use crate::inventory::model::InventoryItem;
use crate::inventory::db::InventoryDB;

pub fn setup_search_button(
    search_btn: &mut Button,
    search_input: &Input,
    log_buffer: &TextBuffer,
    inventory_db: Rc<RefCell<InventoryDB>>,
    items: Rc<RefCell<Vec<InventoryItem>>>,
    item_table: Rc<RefCell<Table>>
) {
    let db_clone = inventory_db;
    let items_clone = items;
    let table_clone = item_table;
    let mut log_buffer_clone = log_buffer.clone();
    let search_input_clone = search_input.clone();
    
    search_btn.set_callback(move |_| {
        let query = search_input_clone.value();
        if query.is_empty() {
            // If search is empty, show all items
            if let Ok(all_items) = db_clone.borrow().get_all_items() {
                *items_clone.borrow_mut() = all_items;
                let count = items_clone.borrow().len();
                table_clone.borrow_mut().set_rows(count as i32);
                table_clone.borrow_mut().redraw();
                log_buffer_clone.append("Showing all items\n");
            }
        } else {
            // Search for items
            match db_clone.borrow().search_items(&query) {
                Ok(search_results) => {
                    *items_clone.borrow_mut() = search_results;
                    let count = items_clone.borrow().len();
                    table_clone.borrow_mut().set_rows(count as i32);
                    table_clone.borrow_mut().redraw();
                    log_buffer_clone.append(&format!("Found {} items matching '{}'\n", count, query));
                },
                Err(e) => {
                    dialog::alert(300, 300, &format!("Error searching: {}", e));
                }
            }
        }
    });
}

pub fn filter_by_category(
    category: &str,
    inventory_db: &InventoryDB,
    items: &mut Rc<RefCell<Vec<InventoryItem>>>,
    item_table: &mut Table
) -> Result<(), rusqlite::Error> {
    let filtered_items = if category == "All" {
        inventory_db.get_all_items()?
    } else {
        inventory_db.get_items_by_category(category)?
    };
    
    *items.borrow_mut() = filtered_items;
    item_table.set_rows(items.borrow().len() as i32);
    item_table.redraw();
    
    Ok(())
}
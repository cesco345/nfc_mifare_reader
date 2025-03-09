use fltk::{
    app,
    button::Button,
    dialog,
    enums::{Align, FrameType, Font, LabelType},
    frame::Frame,
    group::{Group, Tabs},
    input::Input,
    prelude::*,
    table::Table,
    text::{TextBuffer, TextDisplay},
};
use fltk::menu::Choice;
use std::cell::RefCell;
use std::rc::Rc;
use std::fs;
use std::collections::HashSet;

use crate::inventory::ui::utils::ChoiceExt;
use crate::inventory::db::InventoryDB;
use crate::inventory::model::{InventoryItem, create_inventory_item};
use crate::inventory::ui::table::setup_inventory_table;
use crate::inventory::ui::form::ItemForm;
use crate::inventory::ui::utils::format_timestamp;

// Structure to hold the UI components for inventory management
pub struct InventoryUI {
    pub inventory_db: Rc<RefCell<InventoryDB>>,
    item_table: Rc<RefCell<Table>>,
    items: Rc<RefCell<Vec<InventoryItem>>>,
    current_tag_id: Rc<RefCell<Option<String>>>,
}

impl InventoryUI {
    // Create a new instance of the inventory management UI
    pub fn new(db_path: &str) -> Result<Self, rusqlite::Error> {
        // Initialize the database
        let inventory_db = match InventoryDB::new(db_path) {
            Ok(db) => Rc::new(RefCell::new(db)),
            Err(e) => return Err(e),
        };
        
        // Create empty table and items vector
        let item_table = Rc::new(RefCell::new(Table::default()));
        let items = Rc::new(RefCell::new(Vec::new()));
        let current_tag_id = Rc::new(RefCell::new(None));
        
        Ok(InventoryUI {
            inventory_db,
            item_table,
            items,
            current_tag_id,
        })
    }
    fn setup_save_button(
        &self,
        save_btn: &mut Button,
        item_form: &ItemForm,
        log_buffer: &TextBuffer
    ) {
        let db_clone = self.inventory_db.clone();
        let items_clone = self.items.clone();
        let current_tag_clone = self.current_tag_id.clone();
        let table_clone = self.item_table.clone();
        let mut log_buffer_clone = log_buffer.clone();
        let item_form_clone = item_form.clone(); // Clone here to use in callback
        
        save_btn.set_callback(move |_| {
            if let Some(tag_id) = current_tag_clone.borrow().clone() {
                match item_form_clone.get_form_data(&tag_id) { // Use cloned version
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
    
    fn setup_delete_button(
        &self,
        delete_btn: &mut Button,
        item_form: &mut ItemForm,
        log_buffer: &TextBuffer
    ) {
        let db_clone = self.inventory_db.clone();
        let items_clone = self.items.clone();
        let current_tag_clone = self.current_tag_id.clone();
        let table_clone = self.item_table.clone();
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
    
    fn setup_clear_button(
        &self,
        clear_btn: &mut Button,
        item_form: &mut ItemForm,
        log_buffer: &TextBuffer
    ) {
        let current_tag_clone = self.current_tag_id.clone();
        let mut log_buffer_clone = log_buffer.clone();
        let mut item_form_clone = item_form.clone();
        
        clear_btn.set_callback(move |_| {
            item_form_clone.clear();
            *current_tag_clone.borrow_mut() = None;
            log_buffer_clone.append("Form cleared\n");
        });
    }
    
    fn setup_add_button(
        &self,
        add_btn: &mut Button,
        item_form: &mut ItemForm,
        log_buffer: &TextBuffer
    ) {
        let current_tag_clone = self.current_tag_id.clone();
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
    
    fn setup_export_button(
        &self,
        export_btn: &mut Button,
        log_buffer: &TextBuffer
    ) {
        let db_clone = self.inventory_db.clone();
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
    
    fn setup_search_button(
        &self,
        search_btn: &mut Button,
        search_input: &Input,
        log_buffer: &TextBuffer
    ) {
        let db_clone = self.inventory_db.clone();
        let items_clone = self.items.clone();
        let table_clone = self.item_table.clone();
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
    
    // Create the inventory tab in the UI
    pub fn create_tab(&self, tabs: &mut Tabs) {
        let inventory_tab = Group::new(0, 50, 800, 550, "Inventory");
        
        // Create the left panel for the table
        let table_panel = Group::new(10, 60, 380, 530, "");
        
        // Search input
        let search_input = Input::new(10, 60, 280, 30, "Search:");
        let mut search_btn = Button::new(300, 60, 80, 30, "Search");
        
        // Create a table to display inventory items
        let mut table = Table::new(10, 100, 380, 350, "");
        
        // Store the table in our struct
        *self.item_table.borrow_mut() = table.clone();
        
        // Action buttons
        let mut refresh_btn = Button::new(10, 460, 120, 30, "Refresh List");
        let mut add_btn = Button::new(140, 460, 120, 30, "Add Item");
        let mut export_btn = Button::new(270, 460, 120, 30, "Export");
        
        let mut stats_frame = Frame::new(10, 500, 380, 80, "Inventory Stats");
        stats_frame.set_frame(FrameType::EngravedBox);
        stats_frame.set_label_type(LabelType::None);
        
        let mut stats_text = Frame::new(20, 510, 360, 60, "");
        stats_text.set_align(Align::TopLeft | Align::Inside);
        
        table_panel.end();
        
        // Create the right panel for item details
        let detail_panel = Group::new(400, 60, 390, 530, "");
        
        let mut detail_title = Frame::new(400, 60, 390, 30, "Item Details");
        detail_title.set_label_font(Font::HelveticaBold);
        detail_title.set_label_size(18);
        
        // Create item form
        let mut item_form = ItemForm::new(400, 100, 390, 260);
        
        // Action buttons
        let mut save_btn = Button::new(400, 370, 120, 30, "Save Changes");
        let mut delete_btn = Button::new(530, 370, 120, 30, "Delete Item");
        let mut clear_btn = Button::new(660, 370, 120, 30, "Clear Form");
        
        // Event log
        let _log_frame = Frame::new(400, 510, 390, 30, "Event Log");
        let mut log_display = TextDisplay::new(400, 540, 390, 40, "");
        let log_buffer = TextBuffer::default();
        log_display.set_buffer(log_buffer.clone());
        
        detail_panel.end();
        
        // Create clones for the setup_inventory_table closure
        let db_clone = self.inventory_db.clone();
        let items_clone = self.items.clone();
        let current_tag_clone = self.current_tag_id.clone();
        let item_form_clone = item_form.clone();
        let log_buffer_clone = log_buffer.clone();
        
        // Setup table with a FnMut closure
        setup_inventory_table(&mut table, items_clone.clone(), {
            let mut item_form_clone = item_form_clone;
            let mut log_buffer_clone = log_buffer_clone;
            
            move |row_index| {
                let tag_id = items_clone.borrow()[row_index].tag_id.clone();
                *current_tag_clone.borrow_mut() = Some(tag_id.clone());
                
                // Load item details
                if let Ok(Some(item)) = db_clone.borrow().get_item(&tag_id) {
                    // Update form fields
                    item_form_clone.display_item(&item);
                    
                    // Log
                    log_buffer_clone.append(&format!("Loaded details for item: {}\n", item.name));
                }
            }
        });
        
        // Set up refresh button handler
        self.setup_refresh_button(
            &mut refresh_btn,
            &mut stats_text,
            &mut item_form.category_choice,
            &log_buffer
        );
        
        // Set up save button handler
        self.setup_save_button(
            &mut save_btn, 
            &item_form,
            &log_buffer
        );
        
        // Set up delete button handler
        self.setup_delete_button(
            &mut delete_btn,
            &mut item_form,
            &log_buffer
        );
        
        // Set up clear button handler
        self.setup_clear_button(
            &mut clear_btn,
            &mut item_form,
            &log_buffer
        );
        
        // Set up add button handler
        self.setup_add_button(
            &mut add_btn,
            &mut item_form,
            &log_buffer
        );
        
        // Set up export button handler
        self.setup_export_button(
            &mut export_btn,
            &log_buffer
        );
        
        // Set up search button handler
        self.setup_search_button(
            &mut search_btn,
            &search_input,
            &log_buffer
        );
        
        inventory_tab.end();
        tabs.add(&inventory_tab);
        
        // Initialize by loading inventory
        app::add_timeout3(0.1, {
            let mut refresh_btn_clone = refresh_btn.clone();
            move |_| {
                refresh_btn_clone.do_callback();
            }
        });
    }
    
    // Helper methods for setting up buttons (to keep the create_tab method shorter)
    fn setup_refresh_button(
        &self,
        refresh_btn: &mut Button,
        stats_text: &mut Frame,
        category_choice: &mut Choice,
        log_buffer: &TextBuffer
    ) {
        let db_clone = self.inventory_db.clone();
        let items_clone = self.items.clone();
        let table_clone = self.item_table.clone();
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
    
    // Method to update inventory with a scanned tag
    pub fn process_scanned_tag(&self, tag_id: &str) {
        // Check if tag exists in inventory
        match self.inventory_db.borrow().get_item(tag_id) {
            Ok(Some(item)) => {
                // Item exists - increment quantity
                let new_quantity = item.quantity + 1;
                if let Err(e) = self.inventory_db.borrow().update_quantity(tag_id, new_quantity) {
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
                    *self.current_tag_id.borrow_mut() = Some(tag_id.to_string());
                    
                    // This would ideally open a form dialog, but for now we'll use a simple input
                    if let Some(name) = dialog::input(300, 300, "Enter item name:", "") {
                        if !name.is_empty() {
                            // Create basic item
                            let item = create_inventory_item(tag_id, &name, None, 1, None, None);
                            
                            // Save to database
                            if let Err(e) = self.inventory_db.borrow().save_item(&item) {
                                dialog::alert(300, 300, &format!("Error saving item: {}", e));
                                return;
                            }
                            
                            // Add Google Drive sync if enabled
                            // Update APP_CONFIG access depending on your final solution
                            // For Mutex-based approach:
                            if let Ok(config) = crate::config::APP_CONFIG.lock() {
                                if config.gdrive_sync_enabled {
                                    use crate::sync::gdrive_sync::GDriveSync;
                                    let gdrive_sync = GDriveSync::new(&config.gdrive_sync_folder);
                                    match gdrive_sync.export_database(&self.inventory_db.borrow()) {
                                        Ok(_) => println!("Automatically synced database to Google Drive"),
                                        Err(e) => println!("Error auto-syncing to Google Drive: {}", e)
                                    }
                                }
                            }
                            
                            dialog::message(300, 300, &format!("New item '{}' added to inventory.", name));
                            
                            // Refresh the table
                            if let Ok(all_items) = self.inventory_db.borrow().get_all_items() {
                                *self.items.borrow_mut() = all_items;
                                let mut table = self.item_table.borrow_mut();
                                table.set_rows(self.items.borrow().len() as i32);
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
}
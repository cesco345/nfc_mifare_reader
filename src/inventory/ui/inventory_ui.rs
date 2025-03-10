// src/inventory/ui/inventory_ui.rs
use fltk::{
    app,
    button::Button,
    enums::{Align, FrameType, Font, LabelType},
    frame::Frame,
    group::{Group, Tabs},
    input::Input,
    prelude::*,
    table::Table,
    text::{TextBuffer, TextDisplay},
};
use std::cell::RefCell;
use std::rc::Rc;

use crate::inventory::db::InventoryDB;
use crate::inventory::model::InventoryItem;
use crate::inventory::ui::components::form::ItemForm;
use crate::inventory::ui::components::table::setup_inventory_table;
use crate::inventory::ui::handlers::{
    item_handlers::{
        setup_add_button, setup_clear_button, setup_delete_button, 
        setup_refresh_button, setup_save_button
    },
    search_handlers::setup_search_button,
    export_handlers::setup_export_button,
    scan_handlers::process_scanned_tag
};

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
        let mut log_buffer = TextBuffer::default();
        log_display.set_buffer(log_buffer.clone());
        
        detail_panel.end();
        
        // Setup table with a separate closure to avoid borrowing/moving issues
        let db_clone = self.inventory_db.clone();
        let current_tag_clone = self.current_tag_id.clone();
        let items_clone = self.items.clone();
        let mut item_form_clone = item_form.clone();
        let mut log_buffer_clone = log_buffer.clone();
        
        setup_inventory_table(&mut table, items_clone.clone(), move |row_index| {
            let tag_id = items_clone.borrow()[row_index].tag_id.clone();
            *current_tag_clone.borrow_mut() = Some(tag_id.clone());
            
            // Load item details
            if let Ok(Some(item)) = db_clone.borrow().get_item(&tag_id) {
                // Update form fields
                item_form_clone.display_item(&item);
                
                // Log
                log_buffer_clone.append(&format!("Loaded details for item: {}\n", item.name));
            }
        });
        
        // Set up button handlers
        setup_refresh_button(
            &mut refresh_btn,
            &mut stats_text,
            &mut item_form.category_choice,
            &log_buffer,
            self.inventory_db.clone(),
            self.items.clone(),
            self.item_table.clone()
        );
        
        setup_save_button(
            &mut save_btn, 
            &item_form,
            &log_buffer,
            self.inventory_db.clone(),
            self.items.clone(),
            self.current_tag_id.clone(),
            self.item_table.clone()
        );
        
        setup_delete_button(
            &mut delete_btn,
            &mut item_form,
            &log_buffer,
            self.inventory_db.clone(),
            self.items.clone(),
            self.current_tag_id.clone(),
            self.item_table.clone()
        );
        
        setup_clear_button(
            &mut clear_btn,
            &mut item_form,
            &log_buffer,
            self.current_tag_id.clone()
        );
        
        setup_add_button(
            &mut add_btn,
            &mut item_form,
            &log_buffer,
            self.current_tag_id.clone()
        );
        
        setup_export_button(
            &mut export_btn,
            &log_buffer,
            self.inventory_db.clone()
        );
        
        setup_search_button(
            &mut search_btn,
            &search_input,
            &log_buffer,
            self.inventory_db.clone(),
            self.items.clone(),
            self.item_table.clone()
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
    
    // Method to update inventory with a scanned tag
    pub fn process_scanned_tag(&self, tag_id: &str) {
        process_scanned_tag(
            tag_id,
            &self.inventory_db,
            &self.current_tag_id,
            &self.items,
            &self.item_table
        )
    }
}
    
    // Method to update inventory with a sc
// app/init.rs
use fltk::{
    app,
    prelude::*,
    window::Window,
    group::Tabs,
    enums::Align,
};
use std::cell::RefCell;
use std::rc::Rc;

use crate::config;
use crate::app::menu;
use crate::app::events;
use crate::inventory::InventoryUI;
use crate::reader;

pub fn run() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 800, 600, "Mifare Reader Utility");
    
    // Create menu and get the receiver for events
    let (receiver, menu_items) = menu::create_menu(&mut wind);
    
    // Create tabs - positioned just below the menu bar
    let mut tabs = Tabs::new(0, 25, 800, 575, "");
    // Make sure tabs are aligned to the top and visible
    tabs.set_tab_align(Align::Top);
    
    // Load configuration
    let app_config = Rc::new(RefCell::new(config::load_config()));
    
    // Create shared state for keyboard layout selection
    let keyboard_layout = Rc::new(RefCell::new(app_config.borrow().default_keyboard_layout));
    
    // Create card data buffer to share between tabs
    let card_data_buffer = Rc::new(RefCell::new(fltk::text::TextBuffer::default()));
    
    // Create the basic UI tabs first
    crate::ui::create_reader_tab(&mut tabs, keyboard_layout.clone(), card_data_buffer.clone());
    crate::ui::create_conversion_tab(&mut tabs, keyboard_layout.clone());
    crate::ui::create_batch_tab(&mut tabs, keyboard_layout.clone());
    
    // Initialize inventory database
    let inventory_ui = match initialize_inventory_database("inventory.db") {
        Ok(ui) => ui,
        Err(_) => {
            // Error already handled in function
            return;
        }
    };
    
    // Setup import directories
    setup_directories();
    
    tabs.end();
    
    // Force a redraw to ensure UI updates
    app::redraw();
    
    wind.end();
    wind.show();
    
    println!("Main window shown");
    
    // Start the event loop
    events::run_event_loop(
        app,
        receiver,
        keyboard_layout,
        app_config,
        card_data_buffer,
        inventory_ui,
        menu_items
    );
}

fn initialize_inventory_database(db_path: &str) -> Result<Rc<InventoryUI>, ()> {
    match InventoryUI::new(db_path) {
        Ok(ui) => {
            println!("Successfully initialized inventory database");
            let ui_rc = Rc::new(ui);
            // Set the global inventory reference so reader.rs can access it
            reader::set_inventory_ui(&ui_rc);
            Ok(ui_rc)
        },
        Err(e) => {
            println!("Error initializing inventory database: {}", e);
            fltk::dialog::alert(300, 300, &format!("Error initializing inventory database: {}", e));
            Err(())
        }
    }
}

fn setup_directories() {
    // Ensure import directories exist
    let import_dir = "./import";
    let processed_dir = "./processed";
    let error_dir = "./error";
    
    // Create directories if they don't exist
    for dir in &[import_dir, processed_dir, error_dir] {
        if !std::path::Path::new(dir).exists() {
            if let Err(e) = std::fs::create_dir_all(dir) {
                println!("Error creating directory {}: {}", dir, e);
            } else {
                println!("Created directory: {}", dir);
            }
        }
    }
}
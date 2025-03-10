// main.rs
mod ui;
mod reader;
mod utils;
mod batch;
mod config;
mod export;
mod inventory;
mod db_viewer;
mod app;
mod sync;

use fltk::{
    prelude::*,
    window::Window,
    group::Tabs,
    enums::Align,
    menu::{MenuBar, MenuFlag},
    dialog,
};
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

fn main() {
    let app = fltk::app::App::default();
    let mut wind = Window::new(100, 100, 800, 600, "Mifare Reader Utility");
    
    // Create menu
    let mut menu = MenuBar::new(0, 0, 800, 25, "");
    
    // Create a channel for menu events
    let (sender, receiver) = fltk::app::channel::<String>();
    
    // Create clones of the sender for each menu item before using move
    let sender_csv = sender.clone();
    let sender_json = sender.clone();
    let sender_text = sender.clone();
    let sender_log = sender.clone();
    let sender_exit = sender.clone();
    let sender_pref = sender.clone();
    let sender_kb_auto = sender.clone();
    let sender_kb_win = sender.clone();
    let sender_kb_mac = sender.clone();
    let sender_kb_intl = sender.clone();
    let sender_about = sender.clone();
    let sender_import = sender.clone();
    let sender_view_db = sender.clone();
    let sender_check_files = sender.clone();
    let sender_gdrive_export = sender.clone();
    let sender_gdrive_import = sender.clone();
    
    // Add menu items
    menu.add(
        "&File/&Export Data/as &CSV\t",
        fltk::enums::Shortcut::Ctrl | 'e',
        MenuFlag::Normal,
        move |_| { sender_csv.send("export_csv".to_string()); }
    );
    
    menu.add(
        "&File/&Export Data/as &JSON\t",
        fltk::enums::Shortcut::Ctrl | 'j',
        MenuFlag::Normal,
        move |_| { sender_json.send("export_json".to_string()); }
    );
    
    menu.add(
        "&File/&Export Data/as &Text\t",
        fltk::enums::Shortcut::Ctrl | 't',
        MenuFlag::Normal,
        move |_| { sender_text.send("export_text".to_string()); }
    );
    
    menu.add(
        "&File/&Import Data\t",
        fltk::enums::Shortcut::Ctrl | 'i',
        MenuFlag::Normal,
        move |_| { sender_import.send("import_data".to_string()); }
    );
    
    menu.add(
        "&File/&View Database\t",
        fltk::enums::Shortcut::Ctrl | 'd',
        MenuFlag::Normal,
        move |_| { sender_view_db.send("view_database".to_string()); }
    );
    
    menu.add(
        "&File/&Check Import Files\t",
        fltk::enums::Shortcut::Ctrl | 'r',
        MenuFlag::Normal,
        move |_| { sender_check_files.send("check_files".to_string()); }
    );
    
    menu.add(
        "&File/&Google Drive/Export Database\t",
        fltk::enums::Shortcut::None,
        MenuFlag::Normal,
        move |_| { sender_gdrive_export.send("gdrive_export".to_string()); }
    );
    
    menu.add(
        "&File/&Google Drive/Import Database\t",
        fltk::enums::Shortcut::None,
        MenuFlag::Normal,
        move |_| { sender_gdrive_import.send("gdrive_import".to_string()); }
    );
    
    menu.add(
        "&File/&Save Log\t",
        fltk::enums::Shortcut::Ctrl | 's',
        MenuFlag::Normal,
        move |_| { sender_log.send("save_log".to_string()); }
    );
    
    menu.add(
        "&File/E&xit\t",
        fltk::enums::Shortcut::Ctrl | 'q',
        MenuFlag::Normal,
        move |_| { sender_exit.send("exit".to_string()); }
    );
    
    menu.add(
        "&Edit/&Preferences\t",
        fltk::enums::Shortcut::Ctrl | 'p',
        MenuFlag::Normal,
        move |_| { sender_pref.send("preferences".to_string()); }
    );
    
    menu.add(
        "&Edit/&Keyboard Layout/&Auto-detect\t",
        fltk::enums::Shortcut::None,
        MenuFlag::Normal,
        move |_| { sender_kb_auto.send("kb_auto".to_string()); }
    );
    
    menu.add(
        "&Edit/&Keyboard Layout/&Windows\t",
        fltk::enums::Shortcut::None,
        MenuFlag::Normal,
        move |_| { sender_kb_win.send("kb_windows".to_string()); }
    );
    
    menu.add(
        "&Edit/&Keyboard Layout/&Mac US\t",
        fltk::enums::Shortcut::None,
        MenuFlag::Normal,
        move |_| { sender_kb_mac.send("kb_mac_us".to_string()); }
    );
    
    menu.add(
        "&Edit/&Keyboard Layout/Mac &International\t",
        fltk::enums::Shortcut::None,
        MenuFlag::Normal,
        move |_| { sender_kb_intl.send("kb_mac_intl".to_string()); }
    );
    
    menu.add(
        "&Help/&About\t",
        fltk::enums::Shortcut::None,
        MenuFlag::Normal,
        move |_| { sender_about.send("about".to_string()); }
    );
    
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
    ui::create_reader_tab(&mut tabs, keyboard_layout.clone(), card_data_buffer.clone());
    ui::create_conversion_tab(&mut tabs, keyboard_layout.clone());
    ui::create_batch_tab(&mut tabs, keyboard_layout.clone());
    
    // Try to initialize inventory tab with better error handling
    let inventory_ui = match inventory::InventoryUI::new("inventory.db") {
        Ok(ui) => {
            println!("Successfully initialized inventory database");
            let ui_rc = Rc::new(ui);
            // Set the global inventory reference so reader.rs can access it
            reader::set_inventory_ui(&ui_rc);
            ui_rc
        },
        Err(e) => {
            println!("Error initializing inventory database: {}", e);
            dialog::alert(300, 300, &format!("Error initializing inventory database: {}", e));
            // Return early with the basic UI rather than failing completely
            tabs.end();
            // Just let FLTK handle tab selection
            fltk::app::redraw();
            wind.end();
            wind.show();
            
            // Main event loop with no inventory functionality
            while app.wait() {
                if let Some(msg) = receiver.recv() {
                    if msg == "exit" {
                        wind.hide();
                        break;
                    }
                    // Handle other events...
                }
            }
            return;
        }
    };
    
    // Create inventory tab - we reach here only if initialization succeeded
    println!("Adding inventory tab");
    inventory_ui.create_tab(&mut tabs);
    
    tabs.end();
    
    // Ensure the first tab is selected
    println!("Setting active tab");
    
    // Let FLTK handle default tab selection - this is more reliable
    // than trying to explicitly set it with set_value
    
    wind.end();
    
    // Force a redraw to ensure UI updates
    fltk::app::redraw();
    
    wind.show();
    
    println!("Main window shown");
    
    // Create menu items for the event handler
    let menu_items = app::menu::MenuItems {
        keyboard_layout: keyboard_layout.clone(),
        config: app_config.clone(),
        card_buffer: card_data_buffer.clone(),
        inventory_ui: inventory_ui.clone(),
    };
    
    // Run the event loop
    app::events::run_event_loop(
        app,
        receiver,
        keyboard_layout,
        app_config,
        card_data_buffer,
        inventory_ui,
        menu_items
    );
}
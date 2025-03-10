// app/events.rs
use fltk::{
    app,
    prelude::*,
    dialog,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;

use crate::app::menu::MenuItems;
use crate::config;
use crate::db_viewer;
use crate::export;
use crate::sync::gdrive_sync;
use crate::sync::check_for_import_files;


pub fn run_event_loop(
    app: app::App,
    receiver: app::Receiver<String>,
    keyboard_layout: Rc<RefCell<i32>>,
    app_config: Rc<RefCell<config::AppConfig>>,
    card_data_buffer: Rc<RefCell<fltk::text::TextBuffer>>,
    inventory_ui: Rc<crate::inventory::InventoryUI>,
    mut menu_items: MenuItems
) {
    // this is where we update menu items with actual data
    menu_items.keyboard_layout = keyboard_layout;
    menu_items.config = app_config;
    menu_items.card_buffer = card_data_buffer;
    menu_items.inventory_ui = inventory_ui;
    
        
    // entry point and main event loop
    while app.wait() {
        if let Some(msg) = receiver.recv() {
            handle_menu_event(msg, &menu_items);
        }
    }
}

fn handle_menu_event(msg: String, menu_items: &MenuItems) {
    // menu items
    let keyboard_layout = &menu_items.keyboard_layout;
    let config = &menu_items.config;
    let card_buffer = &menu_items.card_buffer;
    let inventory_ui = &menu_items.inventory_ui;
    
    match msg.as_str() {
        "exit" => {
            app::quit();
        },
        "about" => {
            dialog::message(300, 300, "Mifare Reader Utility v0.2.0\n\nA tool for reading and analyzing Mifare card UIDs\nDeveloped with Rust and FLTK\n\nNow with Inventory Management!");
        },
        "preferences" => {
            show_preferences_dialog(keyboard_layout, config);
        },
        "kb_auto" => {
            *keyboard_layout.borrow_mut() = 0;
            config.borrow_mut().default_keyboard_layout = 0;
            let _ = config::save_config(&config.borrow());
        },
        "kb_windows" => {
            *keyboard_layout.borrow_mut() = 1;
            config.borrow_mut().default_keyboard_layout = 1;
            let _ = config::save_config(&config.borrow());
        },
        "kb_mac_us" => {
            *keyboard_layout.borrow_mut() = 2;
            config.borrow_mut().default_keyboard_layout = 2;
            let _ = config::save_config(&config.borrow());
        },
        "kb_mac_intl" => {
            *keyboard_layout.borrow_mut() = 3;
            config.borrow_mut().default_keyboard_layout = 3;
            let _ = config::save_config(&config.borrow());
        },
        "export_csv" => handle_export_csv(card_buffer),
        "export_json" => handle_export_json(card_buffer),
        "export_text" => handle_export_text(card_buffer),
        "view_database" => {
            db_viewer::show_database_viewer(inventory_ui);
        },
        "check_files" => handle_check_files(inventory_ui),
        "gdrive_export" => handle_gdrive_export(inventory_ui, config),
        "gdrive_import" => handle_gdrive_import(inventory_ui, config),
        "import_data" => handle_import_data(inventory_ui),
        "save_log" => {
            match config::save_log(&card_buffer.borrow().text(), &config.borrow()) {
                Ok(msg) => dialog::message(300, 300, &msg),
                Err(e) => dialog::alert(300, 300, &format!("Error saving log: {}", e)),
            }
        },
        _ => {}
    }
}

// handler functions to keep the event loop clean
fn handle_export_csv(card_buffer: &Rc<RefCell<fltk::text::TextBuffer>>) {
    if let Some(path) = dialog::file_chooser("Export as CSV", "*.csv", ".", false) {
        let records = export::parse_display_text(&card_buffer.borrow().text());
        match export::export_data(&records, export::ExportFormat::CSV, &path) {
            Ok(msg) => dialog::message(300, 300, &msg),
            Err(e) => dialog::alert(300, 300, &format!("Error exporting: {}", e)),
        }
    }
}

fn handle_export_json(card_buffer: &Rc<RefCell<fltk::text::TextBuffer>>) {
    if let Some(path) = dialog::file_chooser("Export as JSON", "*.json", ".", false) {
        let records = export::parse_display_text(&card_buffer.borrow().text());
        match export::export_data(&records, export::ExportFormat::JSON, &path) {
            Ok(msg) => dialog::message(300, 300, &msg),
            Err(e) => dialog::alert(300, 300, &format!("Error exporting: {}", e)),
        }
    }
}

fn handle_export_text(card_buffer: &Rc<RefCell<fltk::text::TextBuffer>>) {
    if let Some(path) = dialog::file_chooser("Export as Text", "*.txt", ".", false) {
        let records = export::parse_display_text(&card_buffer.borrow().text());
        match export::export_data(&records, export::ExportFormat::Text, &path) {
            Ok(msg) => dialog::message(300, 300, &msg),
            Err(e) => dialog::alert(300, 300, &format!("Error exporting: {}", e)),
        }
    }
}

fn handle_check_files(inventory_ui: &Rc<crate::inventory::InventoryUI>) {
    let import_dir = "./import";
    let processed_dir = "./processed";
    let error_dir = "./error";
    
    match check_for_import_files(import_dir, processed_dir, error_dir, inventory_ui) {
        Ok(count) => {
            if count > 0 {
                dialog::message(300, 300, &format!("Successfully processed {} files.", count));
            } else {
                dialog::message(300, 300, "No files found to import.");
            }
        },
        Err(e) => {
            dialog::alert(300, 300, &format!("Error processing import files: {}", e));
        }
    }
}

fn handle_gdrive_export(
    inventory_ui: &Rc<crate::inventory::InventoryUI>,
    config: &Rc<RefCell<config::AppConfig>>
) {
    if config.borrow().gdrive_sync_enabled {
        let gdrive_sync = gdrive_sync::GDriveSync::new(&config.borrow().gdrive_sync_folder);
        
        match gdrive_sync.export_database(&inventory_ui.inventory_db.borrow()) {
            Ok(file_path) => {
                dialog::message(300, 300, &format!("Database exported to Google Drive sync folder:\n{}", file_path));
            },
            Err(e) => {
                dialog::alert(300, 300, &format!("Error exporting to Google Drive sync folder: {}", e));
            }
        }
    } else {
        dialog::alert(300, 300, "Google Drive sync is not enabled. Please enable it in preferences.");
    }
}

fn handle_gdrive_import(
    inventory_ui: &Rc<crate::inventory::InventoryUI>,
    config: &Rc<RefCell<config::AppConfig>>
) {
    if config.borrow().gdrive_sync_enabled {
        let gdrive_sync = gdrive_sync::GDriveSync::new(&config.borrow().gdrive_sync_folder);
        
        match gdrive_sync.import_latest_database(&inventory_ui.inventory_db.borrow()) {
            Ok(count) => {
                dialog::message(300, 300, &format!("Successfully imported {} items from Google Drive", count));
            },
            Err(e) => {
                dialog::alert(300, 300, &format!("Error importing from Google Drive: {}", e));
            }
        }
    } else {
        dialog::alert(300, 300, "Google Drive sync is not enabled. Please enable it in preferences.");
    }
}

fn handle_import_data(inventory_ui: &Rc<crate::inventory::InventoryUI>) {
    if let Some(path) = dialog::file_chooser("Import data", "*.{json,csv}", ".", true) {
        if !Path::new(&path).exists() {
            dialog::alert(300, 300, &format!("File does not exist: {}", path));
            return;
        }
        
        match std::fs::read_to_string(&path) {
            Ok(content) => {
                // Check if it's JSON or CSV
                if path.ends_with(".json") {
                    // Import JSON
                    match inventory_ui.inventory_db.borrow().import_json(&content) {
                        Ok(count) => {
                            dialog::message(300, 300, &format!("Successfully imported {} items from JSON.", count));
                        },
                        Err(e) => {
                            dialog::alert(300, 300, &format!("Error importing JSON data: {}", e));
                        }
                    }
                } else {
                    dialog::alert(300, 300, "CSV import is not yet implemented.");
                }
            },
            Err(e) => {
                dialog::alert(300, 300, &format!("Error reading file: {}", e));
            }
        }
    }
}

fn show_preferences_dialog(
    keyboard_layout: &Rc<RefCell<i32>>,
    config: &Rc<RefCell<config::AppConfig>>
) {
    // create the preferences window and its components
    let prefs_win_rc = Rc::new(RefCell::new(fltk::window::Window::new(300, 100, 400, 300, "Preferences")));
    
    // use Rc::borrow_mut() to modify the window
    prefs_win_rc.borrow_mut().make_modal(true);
    
    // it is important to set the end() method to make the window visible 
    let tabs = fltk::group::Tabs::new(10, 10, 380, 240, "");
    
    // this is the general settings tab
    let general_tab = fltk::group::Group::new(10, 35, 380, 215, "General");
    
    let mut save_logs_check = fltk::button::CheckButton::new(20, 45, 200, 25, "Save logs to file");
    save_logs_check.set_checked(config.borrow().save_logs);
    
    let mut log_dir_input = fltk::input::Input::new(140, 75, 240, 25, "Log directory:");
    log_dir_input.set_value(&config.borrow().log_directory);
    
    let _layout_choice_text = fltk::frame::Frame::new(20, 105, 120, 25, "Keyboard Layout:");
    
    let mut layout_choice = fltk::menu::Choice::new(140, 105, 240, 25, "");
    layout_choice.add_choice("Auto-detect");
    layout_choice.add_choice("Windows");
    layout_choice.add_choice("Mac US");
    layout_choice.add_choice("Mac International");
    layout_choice.set_value(config.borrow().default_keyboard_layout);
    
    general_tab.end();
    
    // this is the Google Drive sync tab
    let gdrive_tab = fltk::group::Group::new(10, 35, 380, 215, "Google Drive");
    
    let mut gdrive_enable_check = fltk::button::CheckButton::new(20, 45, 200, 25, "Enable Google Drive sync");
    gdrive_enable_check.set_checked(config.borrow().gdrive_sync_enabled);
    
    let mut gdrive_folder_input = fltk::input::Input::new(140, 75, 200, 25, "Sync folder:");
    gdrive_folder_input.set_value(&config.borrow().gdrive_sync_folder);
    
    let mut gdrive_folder_btn = fltk::button::Button::new(350, 75, 30, 25, "...");
    
    let mut gdrive_folder_input_clone = gdrive_folder_input.clone();
    gdrive_folder_btn.set_callback(move |_| {
        if let Some(path) = dialog::dir_chooser("Select Google Drive sync folder", "", false) {
            gdrive_folder_input_clone.set_value(&path);
        }
    });
    
    // lets the user know how to use Google Drive sync
    let mut gdrive_info_buffer = fltk::text::TextBuffer::default();
    gdrive_info_buffer.set_text("How to use Google Drive sync:\n\n1. Install Google Drive for Desktop\n2. Select a folder inside your Google Drive\n3. Enable sync above and set the folder path\n4. Use Export/Import menu options to sync your database");
    
    let mut gdrive_info = fltk::text::TextDisplay::new(20, 110, 360, 125, "");
    gdrive_info.set_buffer(gdrive_info_buffer);
    
    gdrive_tab.end();
    
    tabs.end();
    
    // these buttons make sure the user can save or cancel their changes
    let mut ok_button = fltk::button::Button::new(220, 260, 80, 30, "OK");
    let mut cancel_button = fltk::button::Button::new(310, 260, 80, 30, "Cancel");
    
    prefs_win_rc.borrow_mut().end();
    prefs_win_rc.borrow_mut().show();
    
    // this is to clone the config and keyboard layout for the button callbacks
    let config_clone_ok = config.clone();
    let keyboard_layout_ok = keyboard_layout.clone();
    
    // this is for cloning the window to hide it after the OK button is clicked
    let prefs_win_ok = prefs_win_rc.clone();
    ok_button.set_callback(move |_| {
        // this config is mutable because we are changing the settings
        let mut config = config_clone_ok.borrow_mut();
        config.save_logs = save_logs_check.is_checked();
        config.log_directory = log_dir_input.value();
        config.default_keyboard_layout = layout_choice.value();
        
        // these are the Google Drive sync settings
        config.gdrive_sync_enabled = gdrive_enable_check.is_checked();
        config.gdrive_sync_folder = gdrive_folder_input.value();
        
        // it creates the Google Drive sync folder if it doesn't exist
        if config.gdrive_sync_enabled {
            let gdrive_path = std::path::Path::new(&config.gdrive_sync_folder);
            if !gdrive_path.exists() {
                if let Err(e) = std::fs::create_dir_all(&config.gdrive_sync_folder) {
                    dialog::alert(300, 300, &format!("Error creating Google Drive sync folder: {}", e));
                }
            }
        }
        
        // time to save the config underscore is used to ignore the result
        let _ = config::save_config(&config);
        
        // updates the keyboard layout and mutable because we are changing it
        *keyboard_layout_ok.borrow_mut() = config.default_keyboard_layout;
        
        prefs_win_ok.borrow_mut().hide();
    });
    
    // this clones the window to hide it after the cancel button is clicked
    let prefs_win_cancel = prefs_win_rc.clone();
    cancel_button.set_callback(move |_| {
        prefs_win_cancel.borrow_mut().hide();
    });
}
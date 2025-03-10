// app/menu.rs
use fltk::{
    app,
    prelude::*,
    menu::{MenuBar, MenuFlag},
};
use std::rc::Rc;
use std::cell::RefCell;

pub struct MenuItems {
    pub keyboard_layout: Rc<RefCell<i32>>,
    pub config: Rc<RefCell<crate::config::AppConfig>>,
    pub card_buffer: Rc<RefCell<fltk::text::TextBuffer>>,
    pub inventory_ui: Rc<crate::inventory::InventoryUI>,
    
}

pub fn create_menu(wind: &mut fltk::window::Window) -> (app::Receiver<String>, MenuItems) {
    // Create menu
    let mut menu = MenuBar::new(0, 0, 800, 25, "");
    
    // Create a channel for menu events
    let (sender, receiver) = app::channel::<String>();
    
    // Add file menu
    add_file_menu(&mut menu, &sender);
    
    // Add edit menu
    add_edit_menu(&mut menu, &sender);
    
    // Add help menu
    add_help_menu(&mut menu, &sender);
    
    // Return the receiver and empty menu items (to be populated later)
    (receiver, MenuItems {
        keyboard_layout: Rc::new(RefCell::new(0)),
        config: Rc::new(RefCell::new(crate::config::AppConfig::default())),
        card_buffer: Rc::new(RefCell::new(fltk::text::TextBuffer::default())),
        inventory_ui: Rc::new(crate::inventory::InventoryUI::new("").unwrap()), // This will be replaced
    })
}

fn add_file_menu(menu: &mut MenuBar, sender: &app::Sender<String>) {
    // Clone sender for each menu item
    let sender_csv = sender.clone();
    let sender_json = sender.clone();
    let sender_text = sender.clone();
    let sender_log = sender.clone();
    let sender_exit = sender.clone();
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
        "&File/&Export to Google Drive\t",
        fltk::enums::Shortcut::Ctrl | 'g',
        MenuFlag::Normal,
        move |_| { sender_gdrive_export.send("gdrive_export".to_string()); }
    );

    menu.add(
        "&File/&Import from Google Drive\t",
        fltk::enums::Shortcut::Ctrl | 'h',  // Using 'h' to avoid conflict with 'g'
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
}

fn add_edit_menu(menu: &mut MenuBar, sender: &app::Sender<String>) {
    let sender_pref = sender.clone();
    let sender_kb_auto = sender.clone();
    let sender_kb_win = sender.clone();
    let sender_kb_mac = sender.clone();
    let sender_kb_intl = sender.clone();
    
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
}

fn add_help_menu(menu: &mut MenuBar, sender: &app::Sender<String>) {
    let sender_about = sender.clone();
    
    menu.add(
        "&Help/&About\t",
        fltk::enums::Shortcut::None,
        MenuFlag::Normal,
        move |_| { sender_about.send("about".to_string()); }
    );
}

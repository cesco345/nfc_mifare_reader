// db_viewer.rs
use fltk::{
    app,
    button::Button,
    dialog,
    prelude::*,
    browser::HoldBrowser,
    window::Window,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::fs::File;
use std::io::Write;
use chrono::DateTime;
use chrono_tz::US::Eastern;

use crate::inventory::ui::actions::InventoryUI;
use crate::inventory::InventoryItem;

pub fn show_database_viewer(inventory_ui: &Rc<InventoryUI>) {
    let mut win = Window::new(100, 100, 1000, 600, "Database Viewer");
    win.make_resizable(true);
    
    // Create a browser widget which is more reliable for displaying tabular data
    let mut browser = HoldBrowser::new(10, 10, 980, 540, "");
    browser.set_column_widths(&[120, 150, 80, 120, 120, 150, 150]);
    browser.set_column_char('\t'); // Use tab as column separator
    
    // Add header row with column titles
    browser.add("@B10@C10Tag ID\tName\tQuantity\tCategory\tLocation\tCreated\tUpdated");
    
    // Get all inventory items
    let items = match inventory_ui.inventory_db.borrow().get_all_items() {
        Ok(items) => items,
        Err(e) => {
            dialog::alert(300, 300, &format!("Error loading inventory: {}", e));
            return;
        }
    };
    
    // Store items in a shared container for callbacks
    let items_data = Rc::new(RefCell::new(items));
    
    // Populate the browser with data
    populate_browser(&mut browser, &items_data.borrow());
    
    // Add buttons
    let mut close_btn = Button::new(900, 560, 90, 30, "Close");
    let mut refresh_btn = Button::new(800, 560, 90, 30, "Refresh");
    let mut export_btn = Button::new(700, 560, 90, 30, "Export CSV");
    
    // Add a status label
    let mut status_lbl = fltk::frame::Frame::new(10, 560, 680, 30, "");
    status_lbl.set_label(&format!("{} items in database", items_data.borrow().len()));
    status_lbl.set_align(fltk::enums::Align::Left | fltk::enums::Align::Inside);
    
    // Export button callback
    let items_for_export = items_data.clone();
    export_btn.set_callback(move |_| {
        if let Some(path) = dialog::file_chooser("Export data to CSV", "*.csv", ".", false) {
            export_to_csv(&items_for_export.borrow(), &path);
        }
    });
    
    // Close button callback
    let mut win_copy = win.clone();
    close_btn.set_callback(move |_| {
        win_copy.hide();
    });
    
    // Refresh button callback
    let inventory_ui_ref = inventory_ui.clone();
    let items_for_refresh = items_data.clone();
    let mut browser_copy = browser.clone();
    let mut status_lbl_copy = status_lbl.clone();
    refresh_btn.set_callback(move |_| {
        if let Ok(updated_items) = inventory_ui_ref.inventory_db.borrow().get_all_items() {
            // Update data
            *items_for_refresh.borrow_mut() = updated_items;
            
            // Repopulate browser
            browser_copy.clear();
            browser_copy.add("@B10@C10Tag ID\tName\tQuantity\tCategory\tLocation\tCreated\tUpdated");
            populate_browser(&mut browser_copy, &items_for_refresh.borrow());
            
            // Update status
            status_lbl_copy.set_label(&format!("{} items in database", items_for_refresh.borrow().len()));
        }
    });
    
    win.end();
    win.show();
    
    // Run until window is closed
    while win.shown() {
        app::wait();
    }
}

// Helper function to populate the browser widget with inventory data
fn populate_browser(browser: &mut HoldBrowser, items: &[InventoryItem]) {
    for item in items {
        // Format the row with tabs between columns
        let row = format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}",
            item.tag_id,
            item.name,
            item.quantity,
            item.category.clone().unwrap_or_default(),
            item.location.clone().unwrap_or_default(),
            format_date(&item.created_at),
            format_date(&item.last_updated)
        );
        browser.add(&row);
    }
}

// Format date for display
fn format_date(timestamp: &str) -> String {
    if let Ok(dt) = DateTime::parse_from_rfc3339(timestamp) {
        // Convert UTC time to Eastern Time
        let eastern_time = dt.with_timezone(&Eastern);
        eastern_time.format("%Y-%m-%d %H:%M").to_string()
    } else {
        timestamp.to_string()
    }
}
fn format_timestamp(timestamp: &str) -> String {
    if let Ok(dt) = DateTime::parse_from_rfc3339(timestamp) {
        // Convert UTC time to Eastern Time
        let eastern_time = dt.with_timezone(&Eastern);
        eastern_time.format("%Y-%m-%d %H:%M:%S").to_string()
    } else {
        timestamp.to_string()
    }
}

// Export data to CSV file
fn export_to_csv(items: &[InventoryItem], path: &str) {
    match File::create(path) {
        Ok(mut file) => {
            // Write CSV header
            let _ = writeln!(file, "tag_id,name,quantity,category,location,created_at,last_updated");
            
            // Write data rows
            for item in items {
                let _ = writeln!(file, 
                    "{},{},{},{},{},{},{}",
                    item.tag_id,
                    item.name.replace(",", "\\,"),
                    item.quantity,
                    item.category.clone().unwrap_or_default().replace(",", "\\,"),
                    item.location.clone().unwrap_or_default().replace(",", "\\,"),
                    item.created_at,
                    item.last_updated
                );
            }
            
            dialog::message(300, 300, &format!("Successfully exported {} items to {}", items.len(), path));
        },
        Err(e) => {
            dialog::alert(300, 300, &format!("Error exporting data: {}", e));
        }
    }
}
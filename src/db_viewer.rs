// db_viewer.rs
use fltk::{
    app,
    prelude::*,
    window::Window,
    table::Table,
    button::Button, 
    dialog,
    frame::Frame,
    group::{Group, Flex, Pack, Scroll},
    draw,
};
use std::cell::RefCell;
use std::rc::Rc;


pub fn show_database_viewer(inventory_ui: &Rc<crate::inventory::InventoryUI>) {
    // Create the main window
    let app = app::App::default();
    let mut win = Window::new(100, 100, 960, 620, "Database Viewer");
    win.make_modal(true);
    
    // Use a flex layout for better resizing behavior
    let mut flex = Flex::new(0, 0, 960, 620, None);
    flex.set_type(fltk::group::FlexType::Column);
    flex.set_margin(10);
    
    // Create a frame for the header
    let mut header = Frame::new(0, 0, 940, 30, "Inventory Database");
    header.set_label_size(18);
    header.set_align(fltk::enums::Align::Center);
    flex.fixed(&header, 30);
    
    // Create a scrollable container for the table
    let mut scroll = Scroll::new(0, 0, 940, 0, None);
    scroll.set_type(fltk::group::ScrollType::Both);
    scroll.set_scrollbar_size(15);
    
    // Create a table for the data
    let mut table = Table::new(0, 0, 940, 500, "");
    table.set_rows(0);
    table.set_row_header(true);
    table.set_row_resize(true);
    table.set_cols(7);
    table.set_col_header(true);
    table.set_col_width(0, 130); // Tag ID
    table.set_col_width(1, 190); // Name
    table.set_col_width(2, 80);  // Quantity
    table.set_col_width(3, 130); // Category
    table.set_col_width(4, 130); // Location
    table.set_col_width(5, 140); // Created
    table.set_col_width(6, 140); // Updated
    
    scroll.end();
    
    // Get data from database
    let items = match inventory_ui.inventory_db.borrow().get_all_items() {
        Ok(items) => items,
        Err(e) => {
            dialog::alert(300, 300, &format!("Error loading inventory: {}", e));
            vec![] // Return empty vector on error
        }
    };

    let items_data = Rc::new(RefCell::new(items));
    let items_clone = items_data.clone();

    // Setup selected row tracking
    let selected_row = Rc::new(RefCell::new(-1));
    let selected_row_clone = selected_row.clone();

    // Set up table drawing
    table.draw_cell(move |_t, ctx, row, col, x, y, w, h| {
        match ctx {
            fltk::table::TableContext::StartPage => draw::set_font(fltk::enums::Font::Helvetica, 14),
            fltk::table::TableContext::ColHeader => {
                draw::draw_rect_fill(x, y, w, h, fltk::enums::Color::from_rgb(220, 220, 220));
                draw::set_draw_color(fltk::enums::Color::Black);
                draw::draw_rect(x, y, w, h);
                draw::set_font(fltk::enums::Font::HelveticaBold, 14);
                let header = match col {
                    0 => "Tag ID",
                    1 => "Name",
                    2 => "Quantity",
                    3 => "Category",
                    4 => "Location",
                    5 => "Created",
                    6 => "Updated",
                    _ => "",
                };
                draw::draw_text2(header, x, y, w, h, fltk::enums::Align::Center);
            },
            fltk::table::TableContext::Cell => {
                let items = items_clone.borrow();
                
                // Determine background color (alternate rows, highlight selected)
                let is_selected = *selected_row_clone.borrow() == row;
                let bg_color = if is_selected {
                    fltk::enums::Color::from_rgb(173, 216, 230) // Light blue for selected row
                } else if row % 2 == 0 {
                    fltk::enums::Color::from_rgb(245, 245, 245) // Light gray for even rows
                } else {
                    fltk::enums::Color::White // White for odd rows
                };
                
                draw::draw_rect_fill(x, y, w, h, bg_color);
                draw::set_draw_color(fltk::enums::Color::Black);
                draw::draw_rect(x, y, w, h);
                
                if row < items.len() as i32 {
                    let item = &items[row as usize];
                    let text = match col {
                        0 => &item.tag_id,
                        1 => &item.name,
                        2 => return draw::draw_text2(&item.quantity.to_string(), x, y, w, h, fltk::enums::Align::Center),
                        3 => return draw::draw_text2(item.category.as_deref().unwrap_or(""), x, y, w, h, fltk::enums::Align::Center),
                        4 => return draw::draw_text2(item.location.as_deref().unwrap_or(""), x, y, w, h, fltk::enums::Align::Center),
                        5 => &item.created_at,
                        6 => &item.last_updated,
                        _ => "",
                    };
                    draw::set_font(fltk::enums::Font::Helvetica, 14);
                    draw::draw_text2(text, x + 5, y, w - 10, h, fltk::enums::Align::Left);
                }
            },
            _ => {}
        }
    });
    
    // Handle table selection
    let selected_row_cb = selected_row.clone();
    table.set_callback(move |t| {
        if app::event() == fltk::enums::Event::Released {
            *selected_row_cb.borrow_mut() = t.callback_row();
            t.redraw();
        }
    });
    
    // Create a pack for buttons at the bottom
    let mut button_flex = Flex::new(0, 0, 940, 40, None);
    button_flex.set_type(fltk::group::FlexType::Row);
    flex.fixed(&button_flex, 40); // Fixed height for button area
    
    // Add count display
    let count_str = format!("{} items in database", items_data.borrow().len());
    let mut count_label = Frame::new(0, 0, 200, 30, count_str.as_str());
    count_label.set_label_size(14);
    button_flex.fixed(&count_label, 200);
    
    // Add a spacer to push buttons to the right
    let mut spacer = Frame::new(0, 0, 30, 30, "");
    
    // Create bright, visible buttons with contrasting colors
    let mut delete_btn = Button::new(0, 0, 0, 30, "Delete");
    delete_btn.set_color(fltk::enums::Color::from_rgb(255, 100, 100)); // Red for delete
    delete_btn.set_label_color(fltk::enums::Color::White);
    button_flex.fixed(&delete_btn, 130);
    
    let mut export_btn = Button::new(0, 0, 0, 30, "Export CSV");
    export_btn.set_color(fltk::enums::Color::from_rgb(100, 200, 100)); // Green for export
    export_btn.set_label_color(fltk::enums::Color::Black);
    button_flex.fixed(&export_btn, 130);
    
    let mut refresh_btn = Button::new(0, 0, 0, 30, "Refresh");
    refresh_btn.set_color(fltk::enums::Color::from_rgb(100, 100, 255)); // Blue for refresh
    refresh_btn.set_label_color(fltk::enums::Color::White);
    button_flex.fixed(&refresh_btn, 130);
    
    let mut close_btn = Button::new(0, 0, 0, 30, "Close");
    close_btn.set_color(fltk::enums::Color::from_rgb(200, 200, 200)); // Gray for close
    close_btn.set_label_color(fltk::enums::Color::Black);
    button_flex.fixed(&close_btn, 130);
    
    button_flex.end();
    flex.end();
    
    // End the window
    win.end();
    win.resizable(&flex);
    
    // Set table rows
    table.set_rows(items_data.borrow().len() as i32);
    
    // After window.end(), set up callbacks:
    {
        let selected_row = selected_row.clone();
        let items_data = items_data.clone();
        let inventory_ui_clone = inventory_ui.clone();
        let mut table_clone = table.clone();
        let mut count_label_clone = count_label.clone();
        
        delete_btn.set_callback(move |_| {
            let selected_row_val = *selected_row.borrow();
            if selected_row_val >= 0 && (selected_row_val as usize) < items_data.borrow().len() {
                let items = items_data.borrow();
                let tag_id = items[selected_row_val as usize].tag_id.clone();
                
                // Ask for confirmation
                if dialog::choice2(300, 300, &format!("Are you sure you want to delete the item with Tag ID '{}'?", tag_id), 
                                "No", "Yes", "") == Some(1) {
                    
                    // Delete the item
                    if let Err(e) = inventory_ui_clone.inventory_db.borrow().delete_item(&tag_id) {
                        dialog::alert(300, 300, &format!("Error deleting item: {}", e));
                    } else {
                        dialog::message(300, 300, "Item deleted successfully");
                        
                        // Refresh the table after deletion
                        if let Ok(updated_items) = inventory_ui_clone.inventory_db.borrow().get_all_items() {
                            drop(items); // Explicitly drop the borrowed reference before mutating
                            *items_data.borrow_mut() = updated_items;
                            table_clone.set_rows(items_data.borrow().len() as i32);
                            
                            // Update the count label
                            let new_count = format!("{} items in database", items_data.borrow().len());
                            count_label_clone.set_label(new_count.as_str());
                            
                            table_clone.redraw();
                        }
                    }
                }
            } else {
                dialog::alert(300, 300, "Please select an item to delete");
            }
        });
    }

    {
        let items_data = items_data.clone();
        export_btn.set_callback(move |_| {
            if let Some(path) = dialog::file_chooser("Export as CSV", "*.csv", ".", false) {
                let items = items_data.borrow();
                let mut csv = String::from("Tag ID,Name,Quantity,Category,Location,Created At,Last Updated\n");
                
                for item in items.iter() {
                    let category = item.category.clone().unwrap_or_default().replace(",", "\\,");
                    let location = item.location.clone().unwrap_or_default().replace(",", "\\,");
                    
                    csv.push_str(&format!(
                        "{},{},{},\"{}\",\"{}\",{},{}\n",
                        item.tag_id,
                        item.name.replace(",", "\\,"),
                        item.quantity,
                        category,
                        location,
                        item.created_at,
                        item.last_updated
                    ));
                }
                
                if let Err(e) = std::fs::write(&path, csv) {
                    dialog::alert(300, 300, &format!("Error writing file: {}", e));
                } else {
                    dialog::message(300, 300, &format!("Data exported to {}", path));
                }
            }
        });
    }

    {
        let items_data = items_data.clone();
        let inventory_ui_clone = inventory_ui.clone();
        let mut table_clone = table.clone();
        let mut count_label_clone = count_label.clone();
        
        refresh_btn.set_callback(move |_| {
            if let Ok(updated_items) = inventory_ui_clone.inventory_db.borrow().get_all_items() {
                *items_data.borrow_mut() = updated_items;
                table_clone.set_rows(items_data.borrow().len() as i32);
                
                // Update the count label
                let new_count = format!("{} items in database", items_data.borrow().len());
                count_label_clone.set_label(new_count.as_str());
                
                table_clone.redraw();
            }
        });
    }

    {
        let mut win_clone = win.clone();
        close_btn.set_callback(move |_| {
            win_clone.hide();
        });
    }

    // Show the window and force a redraw to ensure everything is visible
    win.show();
    win.redraw();
    
    // Force a redraw of the entire application to ensure everything is visible
    app::redraw();

    // Run event loop
    while win.shown() {
        app::wait();
    }
}
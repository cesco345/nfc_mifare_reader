// src/inventory/ui/components/table.rs
use fltk::{prelude::*, table::Table, draw};
use std::cell::RefCell;
use std::rc::Rc;

use crate::inventory::model::InventoryItem;

// Function to set up the inventory table
pub fn setup_inventory_table(
    table: &mut Table,
    items: Rc<RefCell<Vec<InventoryItem>>>,
    mut on_selection: impl FnMut(usize) + 'static
) {
    // Configure table
    table.set_rows(0);
    table.set_row_header(true);
    table.set_row_resize(true);
    table.set_cols(4);
    table.set_col_header(true);
    table.set_col_width(0, 100); // ID Column
    table.set_col_width(1, 150); // Name Column
    table.set_col_width(2, 50);  // Quantity Column
    table.set_col_width(3, 80);  // Category Column
    
    // Set up header drawing callback
    table.draw_cell(move |_t, ctx, row, col, x, y, w, h| {
        match ctx {
            fltk::table::TableContext::StartPage => draw::set_font(fltk::enums::Font::Helvetica, 14),
            fltk::table::TableContext::ColHeader => {
                draw::draw_rect_fill(x, y, w, h, fltk::enums::Color::from_rgb(220, 220, 220));
                draw::set_draw_color(fltk::enums::Color::Black);
                draw::draw_rect(x, y, w, h);
                draw::set_font(fltk::enums::Font::HelveticaBold, 12);
                draw::set_draw_color(fltk::enums::Color::Black);
                
                let header = match col {
                    0 => "Tag ID",
                    1 => "Name",
                    2 => "Qty",
                    3 => "Category",
                    _ => "",
                };
                
                draw::draw_text2(header, x, y, w, h, fltk::enums::Align::Center);
            },
            fltk::table::TableContext::Cell => {
                let items = items.borrow();
                
                if row < items.len() as i32 {
                    let item = &items[row as usize];
                    
                    // Alternate row colors
                    if row % 2 == 0 {
                        draw::draw_rect_fill(x, y, w, h, fltk::enums::Color::from_rgb(245, 245, 245));
                    } else {
                        draw::draw_rect_fill(x, y, w, h, fltk::enums::Color::from_rgb(255, 255, 255));
                    }
                    
                    draw::set_draw_color(fltk::enums::Color::Black);
                    draw::draw_rect(x, y, w, h);
                    
                    let text = match col {
                        0 => &item.tag_id,
                        1 => &item.name,
                        2 => return draw::draw_text2(&item.quantity.to_string(), x, y, w, h, fltk::enums::Align::Center),
                        3 => return draw::draw_text2(item.category.as_deref().unwrap_or(""), x, y, w, h, fltk::enums::Align::Center),
                        _ => "",
                    };
                    
                    draw::set_font(fltk::enums::Font::Helvetica, 12);
                    let padding = 5;
                    draw::draw_text2(text, x + padding, y, w - 2 * padding, h, fltk::enums::Align::Left);
                }
            },
            _ => {}
        }
    });
    
    // Set up row selection callback
    table.set_callback(move |t| {
        if t.callback_context() == fltk::table::TableContext::Cell {
            let row = t.callback_row();
            if row < t.rows() && row >= 0 {
                // Use set_row_selected instead of select_row
                t.set_row_position(row);
                on_selection(row as usize);
            }
        }
    });
}
use fltk::draw;
use fltk::enums::{Align, Color, Font, FrameType};
use fltk::menu::Choice;
use fltk::prelude::MenuExt;

// Helper functions for table drawing
pub fn draw_cell_bg(x: i32, y: i32, w: i32, h: i32, color: Color) {
    draw::push_clip(x, y, w, h);
    draw::draw_rect_fill(x, y, w, h, color);
    draw::pop_clip();
}
pub trait ChoiceExt {
    fn update_categories(&mut self, categories: &[String]);
}
impl ChoiceExt for Choice {
    fn update_categories(&mut self, categories: &[String]) {
        self.clear();
        self.add_choice("Uncategorized");
        for cat in categories {
            self.add_choice(cat);
        }
    }
}

pub fn draw_cell_data(x: i32, y: i32, w: i32, h: i32, data: &str) {
    draw::push_clip(x, y, w, h);
    draw::draw_box(FrameType::FlatBox, x, y, w, h, Color::White);
    
    // Text color
    draw::set_draw_color(Color::Black);
    draw::set_font(Font::Helvetica, 14);
    draw::draw_text2(data, x + 5, y, w - 10, h, Align::Left);
    
    draw::pop_clip();
}

// Helper to format timestamp for display
pub fn format_timestamp(timestamp: &str) -> String {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(timestamp) {
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    } else {
        timestamp.to_string()
    }
}
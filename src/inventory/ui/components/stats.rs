// src/inventory/ui/components/stats.rs
use fltk::{
    prelude::*,
    frame::Frame,
    enums::{Align, FrameType, LabelType},
};
use std::collections::HashSet;
use std::cell::RefCell;
use std::rc::Rc;

use crate::inventory::model::InventoryItem;

pub struct StatsFrame {
    frame: Frame,
    text: Frame,
}

impl StatsFrame {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let mut frame = Frame::new(x, y, w, h, label);
        frame.set_frame(FrameType::EngravedBox);
        frame.set_label_type(LabelType::None);
        
        let mut text = Frame::new(x + 10, y + 10, w - 20, h - 20, "");
        text.set_align(Align::TopLeft | Align::Inside);
        
        StatsFrame {
            frame,
            text,
        }
    }
    
    pub fn update(&mut self, items: &[InventoryItem]) {
        // Calculate statistics
        let total_items = items.len();
        let total_quantity: i32 = items.iter().map(|i| i.quantity).sum();
        let categories: HashSet<_> = items
            .iter()
            .filter_map(|i| i.category.clone())
            .collect();
        
        // Update the text display
        self.text.set_label(&format!(
            "Total Items: {}\nTotal Quantity: {}\nCategories: {}",
            total_items,
            total_quantity,
            categories.len()
        ));
    }
}

impl Clone for StatsFrame {
    fn clone(&self) -> Self {
        StatsFrame {
            frame: self.frame.clone(),
            text: self.text.clone(),
        }
    }
}
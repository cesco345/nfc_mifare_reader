//src/inventory/ui/components/form.rs
use fltk::{
    input::{Input, MultilineInput},
    menu::Choice,
    frame::Frame,
    prelude::*,
};
use std::rc::Rc;
use std::cell::RefCell;
use crate::inventory::model::InventoryItem;
use crate::inventory::ui::utils::format_timestamp;

pub struct ItemForm {
    pub name_input: Input,
    pub quantity_input: Input,
    pub category_choice: Choice,
    pub location_input: Input,
    pub description_input: MultilineInput,
    pub tag_id_display: Frame,
    pub created_display: Frame,
    pub updated_display: Frame,
}
impl Clone for ItemForm {
    fn clone(&self) -> Self {
        ItemForm {
            name_input: self.name_input.clone(),
            quantity_input: self.quantity_input.clone(),
            category_choice: self.category_choice.clone(),
            location_input: self.location_input.clone(),
            description_input: self.description_input.clone(),
            tag_id_display: self.tag_id_display.clone(),
            created_display: self.created_display.clone(),
            updated_display: self.updated_display.clone(),
        }
    }
}
impl ItemForm {
    pub fn new(x: i32, y: i32, w: i32, _h: i32) -> Self {
        let name_input = Input::new(x + 100, y, w - 100, 30, "Name:");
        let quantity_input = Input::new(x + 100, y + 40, w - 100, 30, "Quantity:");
        let category_choice = Choice::new(x + 100, y + 80, w - 100, 30, "Category:");
        let location_input = Input::new(x + 100, y + 120, w - 100, 30, "Location:");
        let description_input = MultilineInput::new(x + 100, y + 160, w - 100, 100, "Description:");
        
        let tag_id_display = Frame::new(x, y + 270, w, 30, "Tag ID: None selected");
        let created_display = Frame::new(x, y + 300, w, 30, "Created: -");
        let updated_display = Frame::new(x, y + 330, w, 30, "Updated: -");
        
        ItemForm {
            name_input,
            quantity_input,
            category_choice,
            location_input,
            description_input,
            tag_id_display,
            created_display,
            updated_display,
        }
    }
    
    pub fn clear(&mut self) {
        self.name_input.set_value("");
        self.quantity_input.set_value("");
        self.category_choice.set_value(0);
        self.location_input.set_value("");
        self.description_input.set_value("");
        self.tag_id_display.set_label("Tag ID: None selected");
        self.created_display.set_label("Created: -");
        self.updated_display.set_label("Updated: -");
    }
    
    pub fn display_item(&mut self, item: &InventoryItem) {
        self.name_input.set_value(&item.name);
        self.quantity_input.set_value(&item.quantity.to_string());
        
        if let Some(cat) = &item.category {
            // Find the category in the dropdown
            for i in 0..self.category_choice.size() {
                if let Some(choice_text) = self.category_choice.text(i) {
                    if choice_text == *cat {
                        self.category_choice.set_value(i);
                        break;
                    }
                }
            }
        } else {
            self.category_choice.set_value(0); // Uncategorized
        }
        
        self.location_input.set_value(&item.location.clone().unwrap_or_default());
        self.description_input.set_value(&item.description.clone().unwrap_or_default());
        
        // Update display fields
        self.tag_id_display.set_label(&format!("Tag ID: {}", item.tag_id));
        self.created_display.set_label(&format!("Created: {}", format_timestamp(&item.created_at)));
        self.updated_display.set_label(&format!("Updated: {}", format_timestamp(&item.last_updated)));
    }
    
    pub fn get_form_data(&self, tag_id: &str) -> Result<InventoryItem, String> {
        // Validate form
        let name = self.name_input.value();
        if name.is_empty() {
            return Err("Item name is required.".to_string());
        }
        
        let quantity_str = self.quantity_input.value();
        let quantity = match quantity_str.parse::<i32>() {
            Ok(q) => q,
            Err(_) => {
                return Err("Quantity must be a valid number.".to_string());
            }
        };
        
        // Get other field values
        let category = if self.category_choice.value() <= 0 {
            None
        } else if let Some(cat_text) = self.category_choice.text(self.category_choice.value()) {
            Some(cat_text)
        } else {
            None
        };
        
        let location = if self.location_input.value().is_empty() {
            None
        } else {
            Some(self.location_input.value())
        };
        
        let description = if self.description_input.value().is_empty() {
            None
        } else {
            Some(self.description_input.value())
        };
        
        // Create a new item
        let item = crate::inventory::model::create_inventory_item(
            tag_id,
            &name,
            description.as_deref(),
            quantity,
            location.as_deref(),
            category.as_deref()
        );
        
        Ok(item)
    }
    
    pub fn update_categories(&mut self, categories: &[String]) {
        self.category_choice.clear();
        self.category_choice.add_choice("Uncategorized");
        for cat in categories {
            self.category_choice.add_choice(cat);
        }
    }
}
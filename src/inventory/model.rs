use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// Define item structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InventoryItem {
    pub tag_id: String,
    pub name: String,
    pub description: Option<String>,
    pub quantity: i32,
    pub location: Option<String>,
    pub category: Option<String>,
    pub last_updated: String,
    pub created_at: String,
}

// Helper to generate ISO timestamp
pub fn generate_timestamp() -> String {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    let seconds = since_epoch.as_secs();
    let millis = since_epoch.subsec_millis();
    
    #[allow(deprecated)]
    let datetime = match chrono::DateTime::from_timestamp(seconds as i64, millis * 1_000_000) {
        Some(dt) => dt.naive_local(),
        None => chrono::NaiveDateTime::from_timestamp_millis(0).unwrap()
    };
    datetime.format("%Y-%m-%dT%H:%M:%S.%fZ").to_string()
}

// Create a new inventory item
pub fn create_inventory_item(
    tag_id: &str, 
    name: &str, 
    description: Option<&str>, 
    quantity: i32, 
    location: Option<&str>, 
    category: Option<&str>
) -> InventoryItem {
    let now = generate_timestamp();
    
    InventoryItem {
        tag_id: tag_id.to_string(),
        name: name.to_string(),
        description: description.map(ToString::to_string),
        quantity,
        location: location.map(ToString::to_string),
        category: category.map(ToString::to_string),
        last_updated: now.clone(),
        created_at: now,
    }
}
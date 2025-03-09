// inventory/db.rs
use rusqlite::{params, Connection, Result};
use std::path::Path;
use std::sync::{Arc, Mutex};

use crate::inventory::model::{InventoryItem, generate_timestamp};

// Database management functions
pub struct InventoryDB {
    conn: Connection,
}

impl InventoryDB {
    // Initialize the database
    pub fn new(db_path: &str) -> Result<Self> {
        let create_new = !Path::new(db_path).exists();
        let conn = Connection::open(db_path)?;
        
        let db = InventoryDB { conn };
        
        // Create tables if this is a new database
        if create_new {
            db.create_tables()?;
        }
        
        Ok(db)
    }
    
    // Create the necessary tables
    fn create_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS inventory (
                tag_id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                quantity INTEGER NOT NULL DEFAULT 0,
                location TEXT,
                category TEXT,
                last_updated TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;
        
        Ok(())
    }
    
    // Add or update an item
    pub fn save_item(&self, item: &InventoryItem) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO inventory (
                tag_id, name, description, quantity, location, category, last_updated, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                item.tag_id,
                item.name,
                item.description,
                item.quantity,
                item.location,
                item.category,
                item.last_updated,
                item.created_at
            ],
        )?;
        
        Ok(())
    }
    
    // Retrieve an item by tag ID
    pub fn get_item(&self, tag_id: &str) -> Result<Option<InventoryItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT tag_id, name, description, quantity, location, category, last_updated, created_at 
             FROM inventory WHERE tag_id = ?"
        )?;
        
        let item_iter = stmt.query_map(params![tag_id], |row| {
            Ok(InventoryItem {
                tag_id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                quantity: row.get(3)?,
                location: row.get(4)?,
                category: row.get(5)?,
                last_updated: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?;
        
        let item = item_iter.into_iter().next().transpose()?;
        Ok(item)
    }
    
    // Get all inventory items
    pub fn get_all_items(&self) -> Result<Vec<InventoryItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT tag_id, name, description, quantity, location, category, last_updated, created_at 
             FROM inventory ORDER BY name"
        )?;
        
        let item_iter = stmt.query_map([], |row| {
            Ok(InventoryItem {
                tag_id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                quantity: row.get(3)?,
                location: row.get(4)?,
                category: row.get(5)?,
                last_updated: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?;
        
        let mut items = Vec::new();
        for item in item_iter {
            items.push(item?);
        }
        
        Ok(items)
    }
    
    // Delete an item
    pub fn delete_item(&self, tag_id: &str) -> Result<bool> {
        let affected = self.conn.execute(
            "DELETE FROM inventory WHERE tag_id = ?",
            params![tag_id],
        )?;
        
        Ok(affected > 0)
    }
    
    // Update quantity of an item
    pub fn update_quantity(&self, tag_id: &str, new_quantity: i32) -> Result<bool> {
        let now = generate_timestamp();
        
        let affected = self.conn.execute(
            "UPDATE inventory SET quantity = ?, last_updated = ? WHERE tag_id = ?",
            params![new_quantity, now, tag_id],
        )?;
        
        Ok(affected > 0)
    }
    
    // Get items by category
    pub fn get_items_by_category(&self, category: &str) -> Result<Vec<InventoryItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT tag_id, name, description, quantity, location, category, last_updated, created_at 
             FROM inventory WHERE category = ? ORDER BY name"
        )?;
        
        let item_iter = stmt.query_map(params![category], |row| {
            Ok(InventoryItem {
                tag_id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                quantity: row.get(3)?,
                location: row.get(4)?,
                category: row.get(5)?,
                last_updated: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?;
        
        let mut items = Vec::new();
        for item in item_iter {
            items.push(item?);
        }
        
        Ok(items)
    }
    
    // Get all categories with counts
    pub fn get_categories(&self) -> Result<Vec<(String, i32)>> {
        let mut stmt = self.conn.prepare(
            "SELECT category, COUNT(*) FROM inventory 
             GROUP BY category ORDER BY category"
        )?;
        
        let category_iter = stmt.query_map([], |row| {
            let category: Option<String> = row.get(0)?;
            let count: i32 = row.get(1)?;
            
            Ok((category.unwrap_or_else(|| "Uncategorized".to_string()), count))
        })?;
        
        let mut categories = Vec::new();
        for category in category_iter {
            categories.push(category?);
        }
        
        Ok(categories)
    }
    
    // Search inventory by name, description, or location
    pub fn search_items(&self, query: &str) -> Result<Vec<InventoryItem>> {
        let search_term = format!("%{}%", query);
        
        let mut stmt = self.conn.prepare(
            "SELECT tag_id, name, description, quantity, location, category, last_updated, created_at 
             FROM inventory 
             WHERE name LIKE ? OR description LIKE ? OR location LIKE ? OR category LIKE ?
             ORDER BY name"
        )?;
        
        let item_iter = stmt.query_map(
            params![&search_term, &search_term, &search_term, &search_term], 
            |row| {
                Ok(InventoryItem {
                    tag_id: row.get(0)?,
                    name: row.get(1)?,
                    description: row.get(2)?,
                    quantity: row.get(3)?,
                    location: row.get(4)?,
                    category: row.get(5)?,
                    last_updated: row.get(6)?,
                    created_at: row.get(7)?,
                })
            }
        )?;
        
        let mut items = Vec::new();
        for item in item_iter {
            items.push(item?);
        }
        
        Ok(items)
    }
    
    // Export inventory as JSON
    pub fn export_json(&self) -> Result<String> {
        let items = self.get_all_items()?;
        let json = serde_json::to_string_pretty(&items)
            .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;
        
        Ok(json)
    }
    
    // Export inventory as CSV
    pub fn export_csv(&self) -> Result<String> {
        let items = self.get_all_items()?;
        
        let mut csv = String::from("Tag ID,Name,Description,Quantity,Location,Category,Last Updated,Created At\n");
        
        for item in items {
            let description = item.description.unwrap_or_default().replace(",", "\\,");
            let location = item.location.unwrap_or_default().replace(",", "\\,");
            let category = item.category.unwrap_or_default().replace(",", "\\,");
            
            csv.push_str(&format!(
                "{},{},\"{}\",{},\"{}\",\"{}\",{},{}\n",
                item.tag_id,
                item.name.replace(",", "\\,"),
                description,
                item.quantity,
                location,
                category,
                item.last_updated,
                item.created_at
            ));
        }
        
        Ok(csv)
    }
    
    // Import inventory from JSON
    pub fn import_json(&self, json: &str) -> Result<usize> {
        let items: Vec<InventoryItem> = serde_json::from_str(json)
            .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;
        
        let mut count = 0;
        for item in items {
            self.save_item(&item)?;
            count += 1;
        }
        
        Ok(count)
    }
}

// Add a function to create a thread-safe version of the inventory DB
pub fn create_thread_safe_db(db: InventoryDB) -> Arc<Mutex<InventoryDB>> {
    Arc::new(Mutex::new(db))
}
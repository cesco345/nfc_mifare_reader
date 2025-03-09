// Export the database module
pub mod db;
// Export the model module
pub mod model;
// Export the UI module
pub mod ui;

// Re-export commonly used items for convenience
pub use db::InventoryDB;
pub use model::{InventoryItem, create_inventory_item};
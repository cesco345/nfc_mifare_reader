pub mod form;
pub mod table;
pub mod stats;

// Re-export components for convenience
pub use form::ItemForm;
pub use table::setup_inventory_table;
pub use stats::StatsFrame;
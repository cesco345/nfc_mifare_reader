// sync/mod.rs
pub mod file_sync;
pub mod gdrive_sync;

// Re-export the core types for convenience
pub use file_sync::FileSync;
pub use gdrive_sync::GDriveSync;

// Function to check for import files (moved from main.rs)
pub fn check_for_import_files(
    import_dir: &str, 
    processed_dir: &str, 
    error_dir: &str, 
    inventory_ui: &std::rc::Rc<crate::inventory::InventoryUI>
) -> Result<usize, String> {
    // Implementation moved from main.rs
    // This would process import files using the inventory UI instance
    file_sync::check_for_import_files(import_dir, processed_dir, error_dir, inventory_ui)
}
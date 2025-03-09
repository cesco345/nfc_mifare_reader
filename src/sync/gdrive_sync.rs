// gdrive_sync.rs - Handles Google Drive synchronization
use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use chrono::Local;
use crate::inventory::InventoryDB;

pub struct GDriveSync {
    sync_folder: String,
}

impl GDriveSync {
    pub fn new(sync_folder: &str) -> Self {
        // Create sync folder if it doesn't exist
        if !Path::new(sync_folder).exists() {
            if let Err(e) = fs::create_dir_all(sync_folder) {
                println!("Error creating Google Drive sync folder: {}", e);
            } else {
                println!("Created Google Drive sync folder: {}", sync_folder);
            }
        }
        
        GDriveSync {
            sync_folder: sync_folder.to_string(),
        }
    }
    
    // Export database to Google Drive sync folder
    pub fn export_database(&self, db: &InventoryDB) -> Result<String, String> {
        // Export the database to JSON
        let json_data = match db.export_json() {
            Ok(data) => data,
            Err(e) => return Err(format!("Failed to export database: {}", e))
        };
        
        // Create a timestamped filename
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = format!("inventory_export_{}.json", timestamp);
        let file_path = Path::new(&self.sync_folder).join(filename);
        
        // Write the JSON data to file
        match fs::write(&file_path, json_data) {
            Ok(_) => {
                println!("Database exported to Google Drive sync folder: {:?}", file_path);
                Ok(file_path.to_string_lossy().to_string())
            },
            Err(e) => Err(format!("Failed to write to Google Drive sync folder: {}", e))
        }
    }
    
    // Import latest database file from Google Drive sync folder
    pub fn import_latest_database(&self, db: &InventoryDB) -> Result<usize, String> {
        match self.find_latest_json_file() {
            Some(file_path) => {
                match fs::read_to_string(&file_path) {
                    Ok(content) => {
                        match db.import_json(&content) {
                            Ok(count) => {
                                println!("Imported {} items from Google Drive sync file: {:?}", count, file_path);
                                Ok(count)
                            },
                            Err(e) => Err(format!("Failed to import from Google Drive sync file: {}", e))
                        }
                    },
                    Err(e) => Err(format!("Failed to read Google Drive sync file: {}", e))
                }
            },
            None => Err("No JSON files found in Google Drive sync folder".to_string())
        }
    }
    
    // Find the latest JSON file in the sync folder
    fn find_latest_json_file(&self) -> Option<PathBuf> {
        let mut latest_file: Option<(PathBuf, std::time::SystemTime)> = None;
        
        if let Ok(entries) = fs::read_dir(&self.sync_folder) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "json") {
                        if let Ok(metadata) = fs::metadata(&path) {
                            if let Ok(modified_time) = metadata.modified() {
                                if latest_file.is_none() || modified_time > latest_file.as_ref().unwrap().1 {
                                    latest_file = Some((path, modified_time));
                                }
                            }
                        }
                    }
                }
            }
        }
        
        latest_file.map(|(path, _)| path)
    }
    
    // Get list of all JSON files in the sync folder
    pub fn list_sync_files(&self) -> io::Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        for entry in fs::read_dir(&self.sync_folder)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "json") {
                files.push(path);
            }
        }
        
        // Sort by modified time (newest first)
        files.sort_by(|a, b| {
            let a_time = fs::metadata(a).and_then(|m| m.modified()).unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH);
            let b_time = fs::metadata(b).and_then(|m| m.modified()).unwrap_or_else(|_| std::time::SystemTime::UNIX_EPOCH);
            b_time.cmp(&a_time)
        });
        
        Ok(files)
    }
}
// file_sync.rs
use notify::{Watcher, RecursiveMode, DebouncedEvent, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::path::{Path, PathBuf};
use std::fs;
use chrono::Local;
use std::thread;
use crate::inventory::InventoryUI;


pub struct FileSync {
    import_path: String,
    processed_path: String,
    error_path: String,
    is_running: bool,
}

impl FileSync {
    pub fn new(
        import_path: &str,
        processed_path: &str,
        error_path: &str
    ) -> Self {
        // Create directories if they don't exist
        for dir in [import_path, processed_path, error_path].iter() {
            if !Path::new(dir).exists() {
                let _ = fs::create_dir_all(dir);
            }
        }
        
        FileSync {
            import_path: import_path.to_string(),
            processed_path: processed_path.to_string(),
            error_path: error_path.to_string(),
            is_running: false,
        }
    }
    
    pub fn start(&mut self) -> Result<(), String> {
        if self.is_running {
            println!("File sync already running");
            return Ok(());
        }
        
        self.is_running = true;
        
        // Process any existing files in the import directory
        if let Ok(entries) = fs::read_dir(&self.import_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if self.should_process_file(&entry.path()) {
                        println!("Found existing file to process: {:?}", entry.path());
                        // Instead of processing directly here, we'll move it to a "pending" directory
                        // and let the main thread handle it on next iteration
                    }
                }
            }
        }
        
        // Set up file watcher that just sends notifications, doesn't process
        let import_path = self.import_path.clone();
        let processed_path = self.processed_path.clone();
        let error_path = self.error_path.clone();
        
        // Create a channel for sending file notifications back to the main thread
        let (tx, _rx) = channel();
        let watcher_tx = tx.clone();
        
        // Create the watcher in its own thread
        thread::spawn(move || {
            let (watch_tx, watch_rx) = channel();
            let mut watcher = match watcher(watch_tx, Duration::from_secs(2)) {
                Ok(w) => w,
                Err(e) => {
                    println!("Error creating watcher: {}", e);
                    return;
                }
            };
            
            if let Err(e) = watcher.watch(&import_path, RecursiveMode::Recursive) {
                println!("Error watching directory: {}", e);
                return;
            }
            
            println!("Watching for new files in: {}", import_path);
            
            loop {
                match watch_rx.recv() {
                    Ok(event) => {
                        match event {
                            DebouncedEvent::Create(path) | DebouncedEvent::Write(path) => {
                                if path.extension().map_or(false, |ext| ext == "json") {
                                    // Allow a small delay to ensure file is fully written
                                    thread::sleep(Duration::from_millis(500));
                                    
                                    println!("New file detected: {:?}", path);
                                    
                                    // Just notify the main thread about the file
                                    let _ = watcher_tx.send(path);
                                }
                            },
                            _ => {}
                        }
                    },
                    Err(e) => {
                        println!("Watch error: {:?}", e);
                        break;
                    }
                }
            }
        });
        
        // Return the receiver for the main thread to poll
        Ok(())
    }
    
    fn should_process_file(&self, path: &Path) -> bool {
        path.extension().map_or(false, |ext| ext == "json")
    }
    
    // New method to get list of files to process
    pub fn get_pending_files(&self) -> Vec<PathBuf> {
        let mut files = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&self.import_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if self.should_process_file(&path) {
                        files.push(path);
                    }
                }
            }
        }
        
        files
    }
    
    // Process a single file and move it to appropriate directory
    pub fn process_file(&self, path: &Path, success: bool) -> Result<(), String> {
        let file_name = match path.file_name() {
            Some(name) => name.to_str().unwrap_or("unknown.json"),
            None => return Err("Invalid file path".to_string())
        };
        
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let new_filename = format!("{}_{}", timestamp, file_name);
        
        let dest_path = if success {
            Path::new(&self.processed_path).join(&new_filename)
        } else {
            Path::new(&self.error_path).join(&new_filename)
        };
        
        if let Err(e) = fs::rename(path, &dest_path) {
            return Err(format!("Error moving file: {}", e));
        }
        
        println!("File moved to: {:?}", dest_path);
        Ok(())
    }
}

pub fn check_for_import_files(
    import_dir: &str,
    processed_dir: &str, 
    error_dir: &str,
    inventory_ui: &std::rc::Rc<crate::inventory::InventoryUI>
) -> Result<usize, String> {
    let file_sync = FileSync::new(import_dir, processed_dir, error_dir);
    let pending_files = file_sync.get_pending_files();
    
    let mut processed_count = 0;
    
    for file_path in pending_files {
        // Process each file
        match std::fs::read_to_string(&file_path) {
            Ok(contents) => {
                // Check if it's JSON (we'll only handle JSON for now)
                if file_path.extension().map_or(false, |ext| ext == "json") {
                    match inventory_ui.inventory_db.borrow().import_json(&contents) {
                        Ok(items_imported) => {
                            // Move file to processed directory
                            if let Err(e) = file_sync.process_file(&file_path, true) {
                                eprintln!("Error moving processed file: {}", e);
                            }
                            processed_count += items_imported;
                        },
                        Err(e) => {
                            eprintln!("Error importing file: {}", e);
                            // Move file to error directory
                            if let Err(e) = file_sync.process_file(&file_path, false) {
                                eprintln!("Error moving error file: {}", e);
                            }
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                // Move file to error directory
                if let Err(e) = file_sync.process_file(&file_path, false) {
                    eprintln!("Error moving error file: {}", e);
                }
            }
        }
    }
    
    Ok(processed_count)
}
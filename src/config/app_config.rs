// app_config.rs
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};

// Define the SyncDirs structure
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SyncDirs {
    pub import_dir: String,
    pub processed_dir: String,
    pub error_dir: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub default_keyboard_layout: i32,
    pub manufacturer_database: HashMap<String, String>,
    pub save_logs: bool,
    pub log_directory: String,
    pub recent_files: Vec<String>,
    pub custom_format_patterns: HashMap<String, String>,
    #[serde(default)]
    pub import_directory: String,
    #[serde(default)]
    pub processed_directory: String,
    #[serde(default)]
    pub error_directory: String,
    // Google Drive sync settings
    #[serde(default)]
    pub gdrive_sync_enabled: bool,
    #[serde(default)]
    pub gdrive_sync_folder: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut manufacturer_db = HashMap::new();
        manufacturer_db.insert("04".to_string(), "NXP Semiconductors".to_string());
        manufacturer_db.insert("05".to_string(), "Infineon Technologies".to_string());
        manufacturer_db.insert("16".to_string(), "Texas Instruments".to_string());
        manufacturer_db.insert("21".to_string(), "EM Microelectronic-Marin SA".to_string());
        manufacturer_db.insert("28".to_string(), "LEGIC Identsystems AG".to_string());
        manufacturer_db.insert("29".to_string(), "Gemplus".to_string());
        manufacturer_db.insert("33".to_string(), "Atmel".to_string());
        manufacturer_db.insert("47".to_string(), "Orga Kartensysteme GmbH".to_string());
        manufacturer_db.insert("49".to_string(), "Inside Technology".to_string());
        manufacturer_db.insert("55".to_string(), "Tönnjes C.A.R.D. International".to_string());
        manufacturer_db.insert("57".to_string(), "Giesecke & Devrient".to_string());
        manufacturer_db.insert("75".to_string(), "HID Global".to_string());
        manufacturer_db.insert("87".to_string(), "Identive".to_string());
        manufacturer_db.insert("95".to_string(), "NXP MIFARE Classic".to_string());
        manufacturer_db.insert("96".to_string(), "NXP MIFARE Plus".to_string());
        manufacturer_db.insert("98".to_string(), "NXP MIFARE DESFire".to_string());
        
        let mut custom_patterns = HashMap::new();
        custom_patterns.insert("*h-!)d-e".to_string(), "Card type 1 with QWERTY encoding".to_string());
        custom_patterns.insert("@h-#d-$h-%d-e".to_string(), "Card type 2 with QWERTY encoding".to_string());
        custom_patterns.insert("*h-e".to_string(), "Card type 3 with QWERTY encoding".to_string());
        
        AppConfig {
            default_keyboard_layout: 0, // Auto-detect
            manufacturer_database: manufacturer_db,
            save_logs: false,
            log_directory: "./logs".to_string(),
            recent_files: Vec::new(),
            custom_format_patterns: custom_patterns,
            import_directory: "./import".to_string(),
            processed_directory: "./processed".to_string(),
            error_directory: "./error".to_string(),
            gdrive_sync_enabled: false,
            gdrive_sync_folder: "./gdrive_sync".to_string(),
        }
    }
}

// This function is redundant with Default implementation, 
// but keeping it for backward compatibility
pub fn new_config() -> AppConfig {
    AppConfig::default()
}

const CONFIG_PATH: &str = "mifare_reader_config.json";

pub fn load_config() -> AppConfig {
    if !Path::new(CONFIG_PATH).exists() {
        let config = AppConfig::default();
        save_config(&config).unwrap_or_else(|err| {
            eprintln!("Error saving default config: {}", err);
        });
        return config;
    }
    
    match fs::read_to_string(CONFIG_PATH) {
        Ok(data) => {
            match serde_json::from_str(&data) {
                Ok(config) => config,
                Err(err) => {
                    eprintln!("Error parsing config file, using defaults: {}", err);
                    AppConfig::default()
                }
            }
        },
        Err(err) => {
            eprintln!("Error reading config file, using defaults: {}", err);
            AppConfig::default()
        }
    }
}

pub fn save_config(config: &AppConfig) -> io::Result<()> {
    let data = serde_json::to_string_pretty(config)?;
    let mut file = fs::File::create(CONFIG_PATH)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

pub fn get_manufacturer(code: &str, config: &AppConfig) -> String {
    if code.len() < 2 {
        return "Unknown (UID too short)".to_string();
    }
    
    let manuf_code = &code[0..2].to_lowercase();
    match config.manufacturer_database.get(manuf_code) {
        Some(name) => name.clone(),
        None => "Unknown manufacturer".to_string(),
    }
}

pub fn add_manufacturer(code: &str, name: &str, config: &mut AppConfig) -> io::Result<()> {
    config.manufacturer_database.insert(code.to_lowercase(), name.to_string());
    save_config(config)
}

pub fn add_custom_pattern(pattern: &str, description: &str, config: &mut AppConfig) -> io::Result<()> {
    config.custom_format_patterns.insert(pattern.to_string(), description.to_string());
    save_config(config)
}

// Save log data to a file
pub fn save_log(log_data: &str, config: &AppConfig) -> io::Result<String> {
    if !config.save_logs {
        return Ok("Logging disabled".to_string());
    }
    
    // Create log directory if it doesn't exist
    if !Path::new(&config.log_directory).exists() {
        fs::create_dir_all(&config.log_directory)?;
    }
    
    // Generate filename with timestamp
    let now = chrono::Local::now();
    let filename = format!("{}/mifare_log_{}.txt", 
        config.log_directory,
        now.format("%Y%m%d_%H%M%S"));
    
    // Write log data to file
    let mut file = fs::File::create(&filename)?;
    file.write_all(log_data.as_bytes())?;
    
    Ok(format!("Log saved to {}", filename))
}
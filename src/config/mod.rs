// config/mod.rs (correct version)
pub mod app_config;

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// A thread-safe version of APP_CONFIG
pub static APP_CONFIG: Lazy<Mutex<app_config::AppConfig>> = Lazy::new(|| {
    Mutex::new(app_config::load_config())
});

// Re-export the core types and functions for convenience
pub use app_config::{
    AppConfig,
    SyncDirs,
    load_config,
    save_config,
    save_log,
    get_manufacturer,
    add_manufacturer,
    add_custom_pattern
};

// For backward compatibility
pub use app_config::new_config;
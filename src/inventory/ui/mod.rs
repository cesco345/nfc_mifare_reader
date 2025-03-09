pub mod table;
pub mod form;
pub mod actions;
pub mod utils;

use std::cell::RefCell;
use std::rc::Rc;
use fltk::group::Tabs;
use crate::inventory::db::InventoryDB;

// Re-export the InventoryUI for convenience
pub use actions::InventoryUI;
// app/mod.rs
pub mod init;
pub mod menu;
pub mod events;

// Re-export the run function for convenience
pub use init::run;
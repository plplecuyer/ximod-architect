//! Data models module

pub mod verify;
mod ximod;

// --- V2 roadmap (skeletons) ---
pub mod bethesda_archive;
pub mod compare;
pub mod condition_text;
pub mod conflicts;
pub mod flags;
pub mod nexus_desc;
pub mod plugin_checks;
pub mod plugin_header;
pub mod simulate;
pub mod strings;
pub mod templates;
pub mod translate;

pub use ximod::*;

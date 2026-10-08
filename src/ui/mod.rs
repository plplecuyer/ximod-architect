//! UI module

pub mod components;
pub mod dialogs;
pub mod docs;
pub mod flag_picker;
pub mod fomod_translation;
pub mod history;
pub mod jobs;
pub mod labels;
pub mod main_window;
pub mod menu;
pub mod preview;
pub mod problems;
pub mod project_strings;
pub mod properties;
pub mod selection;
pub mod tabs;
pub mod theme;
pub mod toasts;
pub mod translation;
pub mod tree;
pub mod validation;
pub mod widgets;
pub mod xml_editor;
pub mod xml_highlight;

// --- V2 roadmap (skeletons) ---
pub mod condition_editor;
pub mod dragdrop;
pub mod foldnav;

pub use main_window::XimodApp;

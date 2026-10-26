//! The detectors the plugin can use and where they live on the computer.

mod download_model_use_case;
mod locate_model_use_case;
mod model_catalog_config;
mod models_folder_policy;

pub use locate_model_use_case::locate_model;
pub use model_catalog_config::{model_named, ModelSpec, MODELS};

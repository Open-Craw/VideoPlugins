use std::path::PathBuf;

use videoplugins_plugin_sdk::{Cancel, Failure};

use super::download_model_use_case::download_model;
use super::model_catalog_config::ModelSpec;
use super::models_folder_policy::models_folder;

/// The file of `model`, downloading it first when it is not there yet (once: the next call finds
/// it). A file of the wrong size is taken for a broken download and fetched again.
pub fn locate_model(model: &ModelSpec, cancel: &Cancel) -> Result<PathBuf, Failure> {
    let folder = models_folder(|name| std::env::var(name).ok()).ok_or_else(|| {
        Failure::status(
            422,
            "no place to keep the model: set VIDEOCRAW_MODELS to a folder",
        )
    })?;
    let file = folder.join("coco-objects").join(model.file);
    let present = std::fs::metadata(&file).is_ok_and(|meta| meta.len() == model.bytes);
    if present {
        return Ok(file);
    }
    cancel.progress(&format!("downloading the {} model", model.name));
    download_model(model, &file).map_err(|message| {
        Failure::status(
            503,
            format!(
                "the {} model could not be downloaded: {message}",
                model.name
            ),
        )
        .retryable(true)
    })?;
    Ok(file)
}

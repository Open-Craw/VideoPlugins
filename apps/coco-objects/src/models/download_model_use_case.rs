use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::time::Duration;

use sha2::{Digest, Sha256};

use super::model_catalog_config::ModelSpec;

/// Downloads `model` to `target`, checking its size and its sum, and puts it in place only when
/// both are right: a download that stops, or a file that is not the one pinned, leaves nothing.
pub fn download_model(model: &ModelSpec, target: &Path) -> Result<(), String> {
    let folder = target
        .parent()
        .ok_or_else(|| format!("{} has no folder", target.display()))?;
    std::fs::create_dir_all(folder).map_err(|error| format!("{}: {error}", folder.display()))?;
    let partial = target.with_extension("partial");
    let outcome = fetch(model, &partial);
    if outcome.is_err() {
        let _ = std::fs::remove_file(&partial);
    }
    outcome?;
    std::fs::rename(&partial, target).map_err(|error| format!("{}: {error}", target.display()))
}

fn fetch(model: &ModelSpec, partial: &Path) -> Result<(), String> {
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(900)))
        .build()
        .into();
    let mut response = agent
        .get(model.url)
        .call()
        .map_err(|error| format!("{}: {error}", model.url))?;
    let mut reader = response.body_mut().as_reader();
    let mut file =
        File::create(partial).map_err(|error| format!("{}: {error}", partial.display()))?;
    let mut hash = Sha256::new();
    let mut buffer = vec![0u8; 64 * 1024];
    let mut bytes = 0u64;
    loop {
        let read = reader
            .read(&mut buffer)
            .map_err(|error| format!("{}: {error}", model.url))?;
        if read == 0 {
            break;
        }
        hash.update(&buffer[..read]);
        file.write_all(&buffer[..read])
            .map_err(|error| format!("{}: {error}", partial.display()))?;
        bytes += read as u64;
    }
    file.flush()
        .map_err(|error| format!("{}: {error}", partial.display()))?;
    if bytes != model.bytes {
        return Err(format!(
            "{} gave {bytes} bytes, expected {}",
            model.url, model.bytes
        ));
    }
    let sum = format!("{:x}", hash.finalize());
    if sum != model.sha256 {
        return Err(format!(
            "{} is not the file this plugin expects (sha256 {sum}, expected {})",
            model.url, model.sha256
        ));
    }
    Ok(())
}

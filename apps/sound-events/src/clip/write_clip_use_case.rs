use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::{json, Value};
use videoplugins_plugin_sdk::Failure;

use super::encode_wav_algorithm::encode_wav;

/// Writes `samples` as a WAV file in `work_dir` and describes it the way a result hands a file
/// over: `{ "workFile": <path>, "mime": "audio/wav" }`. VideoCraw moves it into the result's store.
pub fn write_clip(samples: &[f32], sample_rate: u32, work_dir: &Path) -> Result<Value, Failure> {
    static NUMBER: AtomicU64 = AtomicU64::new(0);
    let file = work_dir.join(format!(
        "sound-events-{}-{}.wav",
        std::process::id(),
        NUMBER.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::write(&file, encode_wav(samples, sample_rate))
        .map_err(|error| Failure::status(500, format!("{}: {error}", file.display())))?;
    Ok(json!({ "workFile": file.to_string_lossy(), "mime": "audio/wav" }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_clip_is_written_in_the_work_folder_and_handed_over() {
        let folder = std::env::temp_dir().join(format!("sound-clip-{}", std::process::id()));
        std::fs::create_dir_all(&folder).unwrap();
        let handed = write_clip(&[0.0, 0.5], 16_000, &folder).unwrap();
        assert_eq!(handed["mime"], "audio/wav");
        let path = handed["workFile"].as_str().unwrap();
        assert!(Path::new(path).starts_with(&folder));
        assert_eq!(std::fs::metadata(path).unwrap().len(), 44 + 4);
    }

    #[test]
    fn a_folder_that_is_not_there_is_a_500_that_names_the_file() {
        let failure = write_clip(&[0.0], 16_000, Path::new("no/such/folder")).unwrap_err();
        assert_eq!(failure.code, Some(json!(500)));
        assert!(failure.message.contains("sound-events-"));
    }
}

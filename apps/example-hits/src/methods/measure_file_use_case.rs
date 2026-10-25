use serde_json::{json, Value};

/// Gives the size in bytes of the stored file passed as `params.audio`: a stored file arrives as an
/// object with an absolute `file` path the plugin can open.
pub fn measure_file(params: &Value) -> Result<Value, String> {
    let file = params["audio"]["file"]
        .as_str()
        .ok_or("audio has no file")?;
    let metadata = std::fs::metadata(file).map_err(|error| format!("{file}: {error}"))?;
    Ok(json!({ "bytes": metadata.len() }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measures_a_file_and_explains_a_missing_one() {
        let path = std::env::temp_dir().join(format!("example-hits-{}.bin", std::process::id()));
        std::fs::write(&path, [0_u8; 42]).unwrap();
        let params = json!({ "audio": { "file": path.to_string_lossy() } });
        assert_eq!(measure_file(&params).unwrap(), json!({ "bytes": 42 }));
        let _ = std::fs::remove_file(&path);

        assert!(measure_file(&params).is_err());
        assert_eq!(measure_file(&json!({})).unwrap_err(), "audio has no file");
    }
}

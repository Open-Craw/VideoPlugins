use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::{json, Value};
use videoplugins_plugin_sdk::Failure;

/// Writes an `rgb24` picture as a PNG in `work_dir` and describes it the way a result hands a file
/// over: `{ "workFile": <path>, "mime": "image/png" }`. VideoCraw moves it into the result's store.
pub fn write_picture(
    rgb: &[u8],
    width: usize,
    height: usize,
    work_dir: &Path,
) -> Result<Value, Failure> {
    static NUMBER: AtomicU64 = AtomicU64::new(0);
    let file = work_dir.join(format!(
        "coco-objects-{}-{}.png",
        std::process::id(),
        NUMBER.fetch_add(1, Ordering::Relaxed)
    ));
    let broken = |error: &dyn std::fmt::Display| {
        Failure::status(500, format!("{}: {error}", file.display()))
    };
    let output = File::create(&file).map_err(|error| broken(&error))?;
    let mut encoder = png::Encoder::new(BufWriter::new(output), width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().map_err(|error| broken(&error))?;
    writer
        .write_image_data(rgb)
        .map_err(|error| broken(&error))?;
    writer.finish().map_err(|error| broken(&error))?;
    Ok(json!({ "workFile": file.to_string_lossy(), "mime": "image/png" }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_picture_is_written_as_a_png_that_reads_back_the_same() {
        let folder = std::env::temp_dir().join(format!("coco-picture-{}", std::process::id()));
        std::fs::create_dir_all(&folder).unwrap();
        let rgb: Vec<u8> = (0..4 * 2 * 3).map(|n| (n * 9) as u8).collect();

        let handed = write_picture(&rgb, 4, 2, &folder).unwrap();

        assert_eq!(handed["mime"], "image/png");
        let path = handed["workFile"].as_str().unwrap();
        assert!(Path::new(path).starts_with(&folder));
        let decoder = png::Decoder::new(std::io::BufReader::new(File::open(path).unwrap()));
        let mut reader = decoder.read_info().unwrap();
        let mut back = vec![0u8; reader.output_buffer_size().unwrap()];
        let info = reader.next_frame(&mut back).unwrap();
        assert_eq!((info.width, info.height), (4, 2));
        assert_eq!(&back[..24], &rgb[..]);
    }

    #[test]
    fn a_folder_that_is_not_there_is_a_500_that_names_the_file() {
        let failure = write_picture(&[0; 3], 1, 1, Path::new("no/such/folder")).unwrap_err();
        assert_eq!(failure.code, Some(json!(500)));
        assert!(failure.message.contains("coco-objects-"));
    }
}

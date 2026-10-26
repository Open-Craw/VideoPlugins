use std::path::Path;
use std::sync::{Mutex, MutexGuard, OnceLock};

use ort::environment::Environment;
use ort::session::Session;
use ort::value::Tensor;
use videoplugins_plugin_sdk::Failure;

/// How many numbers a picture or a phrase becomes.
pub const DIMENSIONS: usize = 768;

/// The image model and the text model, loaded the first time each is asked for and kept until the
/// plugin stops: loading takes far longer than looking.
fn slot(vision: bool) -> MutexGuard<'static, Option<Session>> {
    static VISION: OnceLock<Mutex<Option<Session>>> = OnceLock::new();
    static TEXT: OnceLock<Mutex<Option<Session>>> = OnceLock::new();
    let cell = if vision { &VISION } else { &TEXT };
    cell.get_or_init(|| Mutex::new(None))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn environment() -> Result<&'static Environment, Failure> {
    static ENVIRONMENT: OnceLock<Result<Environment, String>> = OnceLock::new();
    ENVIRONMENT
        .get_or_init(|| {
            ort::init()
                .with_telemetry(false)
                .build()
                .map_err(|error| error.to_string())
        })
        .as_ref()
        .map_err(|message| Failure::status(500, format!("ONNX Runtime did not start: {message}")))
}

fn load(file: &Path) -> Result<Session, Failure> {
    Session::builder(environment()?)
        .and_then(|mut builder| builder.commit_from_file(file))
        .map_err(|error| {
            Failure::status(
                422,
                format!(
                    "the model cannot be loaded from {}: {error}",
                    file.display()
                ),
            )
        })
}

/// The 768 numbers of each of `count` pictures, one after another. `values` holds the pictures
/// as `prepare_picture` makes them, one after another.
pub fn embed_pictures(file: &Path, values: Vec<f32>, count: usize) -> Result<Vec<f32>, Failure> {
    let side = crate::pictures::SIDE;
    let input = Tensor::from_array(([count, 3, side, side], values))
        .map_err(|error| broken("the pictures could not be given", &error))?;
    with_session(true, file, |session| {
        let outputs = session
            .run(ort::inputs!["pixel_values" => input])
            .map_err(|error| broken("it could not look", &error))?;
        numbers_of(&outputs, count)
    })
}

/// The 768 numbers of each of `count` phrases, one after another. `ids` holds the pieces of each
/// phrase, `length` of them, padded.
pub fn embed_texts(
    file: &Path,
    ids: Vec<i64>,
    count: usize,
    length: usize,
) -> Result<Vec<f32>, Failure> {
    let input = Tensor::from_array(([count, length], ids))
        .map_err(|error| broken("the phrases could not be given", &error))?;
    with_session(false, file, |session| {
        let outputs = session
            .run(ort::inputs!["input_ids" => input])
            .map_err(|error| broken("it could not read", &error))?;
        numbers_of(&outputs, count)
    })
}

/// Runs `work` on the image model (`vision`) or the text model, loading it first if need be.
fn with_session(
    vision: bool,
    file: &Path,
    work: impl FnOnce(&mut Session) -> Result<Vec<f32>, Failure>,
) -> Result<Vec<f32>, Failure> {
    let mut loaded = slot(vision);
    if loaded.is_none() {
        *loaded = Some(load(file)?);
    }
    let session = loaded
        .as_mut()
        .ok_or_else(|| Failure::status(500, "the model is not loaded"))?;
    work(session)
}

/// What the model answered for `count` inputs: its `pooler_output`, `DIMENSIONS` numbers each.
fn numbers_of(outputs: &ort::session::SessionOutputs, count: usize) -> Result<Vec<f32>, Failure> {
    let (_, numbers) = outputs["pooler_output"]
        .try_extract_tensor::<f32>()
        .map_err(|error| broken("its answer is not numbers", &error))?;
    if numbers.len() != count * DIMENSIONS {
        return Err(Failure::status(
            422,
            format!(
                "the model gave {} numbers for {count} inputs, and {} were expected",
                numbers.len(),
                count * DIMENSIONS
            ),
        ));
    }
    Ok(numbers.to_vec())
}

fn broken(what: &str, error: &dyn std::fmt::Display) -> Failure {
    Failure::status(500, format!("the model: {what}: {error}"))
}

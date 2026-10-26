use crate::detect::Detection;

/// What was found in one picture of the video, and when that picture is.
#[derive(Debug, Clone, PartialEq)]
pub struct Sample {
    /// The place of the picture among those the video gave.
    pub frame: usize,
    pub time_ms: u64,
    pub duration_ms: u64,
    pub found: Vec<Detection>,
}

/// A class that shows up and stays: from the first picture it is in to the last of the same
/// stretch. If it leaves for long and comes back, that is another appearance.
#[derive(Debug, Clone, PartialEq)]
pub struct Appearance {
    pub class: usize,
    /// The picture where it first shows, and its time in the source.
    pub first_frame: usize,
    pub start_ms: u64,
    /// When the last picture it is in stops being shown.
    pub end_ms: u64,
    /// The surest the detector was of it in any picture of the stretch.
    pub confidence: f32,
    /// The most of this class seen together in one picture.
    pub peak_count: usize,
    /// In how many pictures it was seen.
    pub samples: usize,
    /// Where it is in the first picture (a box for each one of the class).
    pub first_boxes: Vec<Detection>,
}

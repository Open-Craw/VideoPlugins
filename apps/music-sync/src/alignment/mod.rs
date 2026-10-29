//! Telling where one sound lies inside another: the delay that lines a music up with the sound of
//! a video, how sure that is, and whether the two drift apart over their length.

mod correlate_algorithm;
mod find_alignment_use_case;
mod measure_drift_use_case;

pub use find_alignment_use_case::{find_alignment, AlignmentOptions};
pub use measure_drift_use_case::measure_drift_ms;

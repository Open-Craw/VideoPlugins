//! Joining what was found picture by picture into appearances: when a class shows up and for how
//! long it stays.

mod appearance_contract;
mod track_appearances_algorithm;

pub use appearance_contract::Sample;
pub use track_appearances_algorithm::track_appearances;

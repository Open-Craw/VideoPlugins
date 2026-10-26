//! Looking at one picture: fitting it for the detector, running the detector, and turning its
//! answer into boxes.

mod decode_output_algorithm;
mod detect_objects_use_case;
mod detection_contract;
mod detector_store;
mod non_max_suppression_algorithm;
mod prepare_input_algorithm;

pub use detect_objects_use_case::detect_objects;
pub use detection_contract::Detection;

//! The picture that shows an appearance: the first frame with the boxes of the class drawn on it,
//! as a PNG handed to the result.

mod draw_boxes_algorithm;
mod write_picture_use_case;

pub use draw_boxes_algorithm::draw_boxes;
pub use write_picture_use_case::write_picture;

//! The protocol over standard input and output: one JSON message per line in, one per line out.

mod serve_lines_handler;

pub use serve_lines_handler::serve_lines;

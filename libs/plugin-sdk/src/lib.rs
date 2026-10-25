//! The seed of the Rust SDK for VideoCraw plugins: it speaks the protocol (see
//! docs/plugin-protocol.md) so a plugin only writes its actions.
//!
//! ```no_run
//! use videoplugins_plugin_sdk::{serve_stdio, Failure};
//!
//! serve_stdio(|action, params, cancel| match action {
//!     "echo" => Ok(params.clone()),
//!     other => Err(Failure::status(404, format!("no action {other:?}"))),
//! });
//! ```
//!
//! What it does: answers `hello`, runs every call on its own thread (so a plugin that declares a
//! `concurrency` above 1 takes that many calls at once), tells a call that the host cancelled it,
//! turns a returned [`Failure`] into an `error` message with its code, retry hint and details, turns
//! a panic into a 500, and stops at `shutdown`. What the plugin decides stays the plugin's: the SDK
//! never reads a result to find out whether it failed.

mod serve;

pub use serve::{serve_lines, serve_stdio, Cancel, Failure};

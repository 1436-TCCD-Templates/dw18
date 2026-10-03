//! # DW18 — Supply Manifest
//!
//! ```text
//! Manifest total: $48.00
//! ```
//!
//! `cargo run` reads `manifest.csv` from the project folder.
//!
//! Do not edit `src/main.rs`; the bugs are in the library files.

pub mod manifest;

/// Formats an amount in cents as dollars.
///
/// ```
/// assert_eq!(dw18::format_dollars(4800), "$48.00");
/// ```
pub fn format_dollars(cents: u32) -> String {
    format!("${:.2}", cents as f64 / 100.0)
}

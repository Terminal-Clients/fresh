//! Compatibility shim replacing the unmaintained `paste` crate.
//!
//! `paste` is pulled in transitively by `metal` (via `wgpu-hal`) and is never
//! used directly by Fresh. RUSTSEC-2024-0436 reports it as unmaintained; its
//! maintained successor is [`pastey`], which exposes the same macros under the
//! same names.
//!
//! The workspace root patches `paste` to this crate, so dependents keep their
//! existing `paste::paste!` / `paste::item!` spellings while the actual
//! implementation is `pastey`. Keep the version at the last upstream `paste`
//! release so the patch satisfies `paste ^1` requirements.

pub use pastey::*;

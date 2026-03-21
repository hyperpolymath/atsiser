// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//
// atsiser library API.
//
// Provides programmatic access to the atsiser pipeline:
// 1. Load and validate an atsiser.toml manifest
// 2. Parse C headers to extract function signatures
// 3. Generate ATS2 viewtype wrappers with linear type proofs
// 4. Drive ATS2 compilation back to C
//
// The library is used by the atsiser CLI and can also be embedded in other
// tools (e.g., iseriser, the meta-framework for the -iser family).

pub mod abi;
pub mod codegen;
pub mod manifest;

pub use abi::{ATSFunction, ATSModule, ATSParam, LinearPtr, MemorySafetyProof, OwnershipPattern, Viewtype};
pub use codegen::ats_gen;
pub use codegen::compiler;
pub use codegen::parser;
pub use manifest::{load_manifest, validate, Manifest};

/// Convenience: load, validate, and generate all ATS2 artifacts in one call.
///
/// # Arguments
///
/// * `manifest_path` — Path to the atsiser.toml manifest
/// * `output_dir` — Directory where generated .sats/.dats files will be written
///
/// # Errors
///
/// Returns an error if manifest loading, validation, or generation fails.
pub fn generate(manifest_path: &str, output_dir: &str) -> anyhow::Result<()> {
    let m = load_manifest(manifest_path)?;
    validate(&m)?;
    codegen::generate_all(&m, output_dir)?;
    Ok(())
}

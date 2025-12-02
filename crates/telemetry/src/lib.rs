
//! Telemetry integration for the Beep workspace.
//!
//! This crate wires up OpenTelemetry (OTLP) tracing and metrics and
//! provides a small public surface used by other crates to initialize
//! telemetry. 
//!

pub mod telemetry;
pub mod domain;

pub use telemetry::{init, OtelGuard};

pub use domain::models::errors::TelemetryError;
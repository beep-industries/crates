
pub mod telemetry;
pub mod domain;

pub use telemetry::{init, OtelGuard};

pub use domain::models::errors::TelemetryError;
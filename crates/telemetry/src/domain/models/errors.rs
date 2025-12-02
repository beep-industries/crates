/// Errors produced by telemetry initialization and shutdown.
#[derive(Debug, thiserror::Error)]
pub enum TelemetryError {
    /// Errors originating from the OpenTelemetry SDK or exporters.
    #[error("OpenTelemetryError: {0}")]
    OpenTelemetry(String),
}
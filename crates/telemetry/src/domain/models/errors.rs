#[derive(Debug, thiserror::Error)]
pub enum TelemetryError {
    #[error("OpenTelemetryError: {0}")]
    OpenTelemetry(String),
}
use clap::Parser;

/// SpiceDb client configuration
#[derive(Debug, Clone, Parser)]
pub struct SpiceDbConfig {
    /// The endpoint URL (e.g., "grpc.authzed.com:443" or "localhost:50051")
    #[arg(
        long = "spicedb-endpoint",
        env = "SPICEDB_ENDPOINT",
        default_value = "localhost:50051"
    )]
    pub endpoint: String,

    /// The preshared key for authentication
    #[arg(long = "spicedb-token", env = "SPICEDB_TOKEN")]
    pub token: Option<String>,
}

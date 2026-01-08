use clap::Parser;

/// Configuration for connecting to a SpiceDB instance.
///
/// `SpiceDbConfig` contains the necessary connection parameters for establishing
/// a gRPC connection to SpiceDB. It supports configuration via environment variables
/// or command-line arguments using `clap`.
///
/// # Configuration Methods
///
/// You can configure SpiceDB connection in three ways:
///
/// 1. **Directly in code** - Create the struct manually
/// 2. **Environment variables** - Set `SPICEDB_ENDPOINT` and `SPICEDB_TOKEN`
/// 3. **Command-line arguments** - Use `--spicedb-endpoint` and `--spicedb-token`
///
/// # Examples
///
/// ## Manual configuration
///
/// ```
/// use authz::SpiceDbConfig;
///
/// let config = SpiceDbConfig {
///     endpoint: "localhost:50051".to_string(),
///     token: Some("your-preshared-key".to_string()),
/// };
/// ```
///
/// ## From environment variables
///
/// ```bash
/// export SPICEDB_ENDPOINT="grpc.authzed.com:443"
/// export SPICEDB_TOKEN="your-preshared-key"
/// ```
///
/// ```no_run
/// use authz::SpiceDbConfig;
/// use clap::Parser;
///
/// let config = SpiceDbConfig::parse();
/// ```
///
/// ## From command-line arguments
///
/// ```bash
/// cargo run -- --spicedb-endpoint localhost:50051 --spicedb-token mykey
/// ```
///
/// # Security Note
///
/// The `token` field contains a sensitive preshared key. Ensure it is:
/// - Never hardcoded in version control
/// - Stored securely (e.g., environment variables, secrets manager)
/// - Transmitted only over secure connections (TLS/SSL)
#[derive(Debug, Clone, Parser)]
pub struct SpiceDbConfig {
    /// The SpiceDB endpoint URL.
    ///
    /// This should be the gRPC endpoint of your SpiceDB server, including the port.
    /// The scheme (http:// or https://) is optional and will be added automatically
    /// if not present.
    ///
    /// # Examples
    ///
    /// - `"localhost:50051"` - Local development
    /// - `"grpc.authzed.com:443"` - AuthZed managed service
    /// - `"spicedb.example.com:443"` - Custom deployment
    ///
    /// # Environment Variable
    ///
    /// Can be set via `SPICEDB_ENDPOINT` environment variable.
    ///
    /// # Default
    ///
    /// Defaults to `"localhost:50051"` if not specified.
    #[arg(
        long = "spicedb-endpoint",
        env = "SPICEDB_ENDPOINT",
        default_value = "localhost:50051"
    )]
    pub endpoint: String,

    /// The preshared key for authenticating with SpiceDB.
    ///
    /// This is a secret token used to authenticate your application with the
    /// SpiceDB server. If `None`, the connection will be made without authentication
    /// (useful for local development with SpiceDB running in insecure mode).
    ///
    /// # Security
    ///
    /// This token grants access to your authorization data. Keep it secure:
    /// - Never commit it to version control
    /// - Use environment variables or a secrets manager
    /// - Rotate it regularly
    /// - Use different tokens for different environments
    ///
    /// # Environment Variable
    ///
    /// Can be set via `SPICEDB_TOKEN` environment variable.
    ///
    /// # Examples
    ///
    /// ```
    /// use authz::SpiceDbConfig;
    ///
    /// // With authentication
    /// let secure_config = SpiceDbConfig {
    ///     endpoint: "grpc.authzed.com:443".to_string(),
    ///     token: Some("tc_my_secret_token_abc123".to_string()),
    /// };
    ///
    /// // Without authentication (local dev only)
    /// let local_config = SpiceDbConfig {
    ///     endpoint: "localhost:50051".to_string(),
    ///     token: None,
    /// };
    /// ```
    #[arg(long = "spicedb-token", env = "SPICEDB_TOKEN")]
    pub token: Option<String>,
}

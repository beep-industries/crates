//! # beep-authz
//!
//! A Rust authorization library with SpiceDB integration for fine-grained permissions.
//!
//! This crate provides a high-level, type-safe interface to [SpiceDB](https://github.com/authzed/spicedb),
//! a Google Zanzibar-inspired authorization system. It enables relationship-based access control
//! (ReBAC) for your Rust applications with minimal boilerplate.
//!
//! ## Features
//!
//! - **SpiceDB Integration** - Native support for SpiceDB/AuthZed with gRPC
//! - **Type Safety** - Strongly-typed permissions and objects
//! - **Async/Await** - Built on Tokio for high-performance async operations
//! - **Easy to Use** - Simple API for checking permissions
//!
//! ## Quick Start
//!
//! ```no_run
//! use authz::{SpiceDbRepository, SpiceDbConfig, SpiceDbObject, Permissions};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Configure connection to SpiceDB
//!     let config = SpiceDbConfig {
//!         endpoint: "localhost:50051".to_string(),
//!         token: Some("your-preshared-key".to_string()),
//!     };
//!
//!     // Create repository
//!     let authz = SpiceDbRepository::new(config).await?;
//!
//!     // Check permissions
//!     let result = authz.check_permissions(
//!         SpiceDbObject::Channel("channel-123".to_string()),
//!         Permissions::ViewChannels,
//!         SpiceDbObject::User("user-456".to_string()),
//!     ).await;
//!
//!     if result.has_permissions() {
//!         println!("Access granted!");
//!     } else {
//!         println!("Access denied!");
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Permission Checking
//!
//! The main functionality is provided by [`SpiceDbRepository`], which offers two methods
//! for checking permissions:
//!
//! - [`SpiceDbRepository::check_permissions`] - High-level, type-safe API
//! - [`SpiceDbRepository::check_permissions_raw`] - Lower-level API for advanced use cases
//!
//! ## Permission Types
//!
//! The [`Permissions`] enum defines all available permission types:
//!
//! - Administrator, ManageServer, ManageRoles
//! - CreateInvitation, ManageChannels, ManageWebhooks
//! - ViewChannels, SendMessages, AttachFiles
//! - ManageNicknames, ChangeNickname, ManageMessages
//!
//! ## Object Types
//!
//! The [`SpiceDbObject`] enum represents different resource types:
//!
//! - `Server` - A server/workspace
//! - `Channel` - A communication channel
//! - `User` - A user/subject
//! - `PermissionOverride` - A permission override rule
//!
//! ## Configuration
//!
//! Configure your SpiceDB connection using [`SpiceDbConfig`], which supports:
//! - Manual configuration
//! - Environment variables (`SPICEDB_ENDPOINT`, `SPICEDB_TOKEN`)
//! - Command-line arguments (via clap)
//!
//! ## Error Handling
//!
//! The crate defines [`AuthorizationError`] for authorization failures:
//! - `Unauthorized` - Permission denied
//! - `ConnectionError` - Failed to connect to SpiceDB
//!
//! ## Examples
//!
//! ### Checking Administrative Access
//!
//! ```no_run
//! # use authz::{SpiceDbRepository, SpiceDbObject, Permissions};
//! # async fn example(repo: SpiceDbRepository) {
//! let is_admin = repo.check_permissions(
//!     SpiceDbObject::Server("my-server".to_string()),
//!     Permissions::Administrator,
//!     SpiceDbObject::User("user-123".to_string()),
//! ).await.has_permissions();
//!
//! if is_admin {
//!     // Grant full access
//! }
//! # }
//! ```
//!
//! ### Using Result for Error Propagation
//!
//! ```no_run
//! # use authz::{SpiceDbRepository, SpiceDbObject, Permissions, AuthorizationError};
//! # async fn example(repo: SpiceDbRepository) -> Result<(), AuthorizationError> {
//! repo.check_permissions(
//!     SpiceDbObject::Channel("private".to_string()),
//!     Permissions::SendMessages,
//!     SpiceDbObject::User("user-456".to_string()),
//! ).await.result()?;
//!
//! // Code here only runs if permission is granted
//! println!("User can send messages");
//! # Ok(())
//! # }
//! ```

// Include the generated protobuf code
pub mod google {
    pub mod rpc {
        tonic::include_proto!("google.rpc");
    }
}

pub mod authzed {
    pub mod api {
        pub mod v1 {
            tonic::include_proto!("authzed.api.v1");
        }
    }
}

// Re-export commonly used types for convenience
pub use authzed::api::v1::{
    experimental_service_client::ExperimentalServiceClient,
    permissions_service_client::PermissionsServiceClient,
    schema_service_client::SchemaServiceClient, watch_service_client::WatchServiceClient,
};
use thiserror::Error;

/// Configuration module for SpiceDB connection settings.
///
/// Contains [`SpiceDbConfig`] for configuring endpoint and authentication.
pub mod config;

/// gRPC authentication interceptor for SpiceDB.
///
/// Internal module that handles token-based authentication for gRPC requests.
pub mod grpc_auth;

/// Object type definitions for SpiceDB resources.
///
/// Contains [`SpiceDbObject`] enum representing different resource types.
pub mod object;

/// Permission types and authorization result handling.
///
/// Contains [`Permissions`] enum and [`AuthorizationResult`] for working with
/// permission check results.
pub mod permission;

/// SpiceDB repository and client implementation.
///
/// Contains [`SpiceDbRepository`] which provides the main API for checking permissions.
pub mod spicedb;

// Re-export main types for convenience
pub use config::SpiceDbConfig;
pub use object::SpiceDbObject;
pub use permission::{AuthorizationResult, Permissions};
pub use spicedb::SpiceDbRepository;

/// Errors that can occur during authorization operations.
///
/// # Variants
///
/// - [`Unauthorized`](AuthorizationError::Unauthorized) - The subject does not have permission to access the resource
/// - [`ConnectionError`](AuthorizationError::ConnectionError) - Failed to establish or maintain connection to SpiceDB
#[derive(Debug, Error)]
pub enum AuthorizationError {
    /// The subject is not allowed to access the requested resource.
    ///
    /// This error is returned when a permission check fails, indicating that
    /// the subject does not have the necessary permission on the resource.
    #[error("You are not allowed to access this resource")]
    Unauthorized,

    /// Failed to connect to the SpiceDB server.
    ///
    /// This error occurs when the gRPC connection to SpiceDB cannot be established
    /// or when communication fails.
    #[error("Could not connect to spice db: {msg}")]
    ConnectionError {
        /// Details about the connection failure
        msg: String,
    },
}

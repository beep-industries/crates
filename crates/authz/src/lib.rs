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

pub mod config;
pub mod grpc_auth;
pub mod object;
pub mod permission;
pub mod spicedb;

#[derive(Debug, Error)]
pub enum AuthorizationError {
    #[error("You are not allowed to access this resource")]
    Unauthorized,

    #[error("Could not connect to spice db: {msg}")]
    ConnectionError { msg: String },
}

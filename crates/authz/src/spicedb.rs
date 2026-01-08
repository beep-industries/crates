use std::sync::Arc;

use tokio::sync::RwLock;
use tonic::{service::interceptor::InterceptedService, transport::Channel};

use crate::{
    AuthorizationError, PermissionsServiceClient,
    authzed::api::v1::{
        CheckPermissionRequest, CheckPermissionResponse, ObjectReference, SubjectReference,
        check_permission_response::Permissionship,
    },
    config::SpiceDbConfig,
    grpc_auth::AuthInterceptor,
    object::SpiceDbObject,
    permission::{AuthorizationResult, Permissions},
};

/// Main SpiceDB client for performing authorization checks.
///
/// `SpiceDbRepository` provides a high-level interface to interact with SpiceDB,
/// a Google Zanzibar-inspired authorization system. It handles connection management,
/// authentication, and permission checking operations.
///
/// # Architecture
///
/// The repository maintains a gRPC connection to a SpiceDB server and provides
/// methods to check whether a subject (e.g., a user) has a specific permission
/// on a resource (e.g., a channel or server).
///
/// # Examples
///
/// ```no_run
/// use authz::{SpiceDbRepository, SpiceDbConfig, SpiceDbObject, Permissions};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = SpiceDbConfig {
///     endpoint: "localhost:50051".to_string(),
///     token: Some("your-token".to_string()),
/// };
///
/// let repo = SpiceDbRepository::new(config).await?;
///
/// // Check if user can view a channel
/// let result = repo.check_permissions(
///     SpiceDbObject::Channel("channel-123".to_string()),
///     Permissions::ViewChannels,
///     SpiceDbObject::User("user-456".to_string()),
/// ).await;
///
/// if result.has_permissions() {
///     println!("Access granted!");
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct SpiceDbRepository {
    permissions:
        Arc<RwLock<PermissionsServiceClient<InterceptedService<Channel, AuthInterceptor>>>>,
}

impl SpiceDbRepository {
    /// Creates a new SpiceDB client with the given configuration.
    ///
    /// This establishes a gRPC connection to the SpiceDB server and sets up
    /// authentication using the provided token.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration containing the endpoint URL and optional authentication token
    ///
    /// # Returns
    ///
    /// Returns `Ok(SpiceDbRepository)` on successful connection, or an
    /// `AuthorizationError::ConnectionError` if the connection fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use authz::{SpiceDbRepository, SpiceDbConfig};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = SpiceDbConfig {
    ///     endpoint: "localhost:50051".to_string(),
    ///     token: Some("somerandomkey".to_string()),
    /// };
    ///
    /// let repo = SpiceDbRepository::new(config).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The endpoint URL is invalid
    /// - The connection to SpiceDB cannot be established
    /// - Network issues prevent communication with the server
    pub async fn new(config: SpiceDbConfig) -> Result<Self, AuthorizationError> {
        let channel = Self::create_channel(&config).await?;

        // Always use an interceptor, even if token is empty
        let token = config.token.unwrap_or_default();

        let interceptor = AuthInterceptor::new(token);
        let permissions = Arc::new(RwLock::new(PermissionsServiceClient::with_interceptor(
            channel.clone(),
            interceptor,
        )));

        Ok(Self { permissions })
    }

    async fn create_channel(config: &SpiceDbConfig) -> Result<Channel, AuthorizationError> {
        // Add http:// scheme if not present
        let endpoint_url =
            if config.endpoint.starts_with("http://") || config.endpoint.starts_with("https://") {
                config.endpoint.clone()
            } else {
                format!("http://{}", config.endpoint)
            };

        let endpoint = Channel::from_shared(endpoint_url.clone())
            .map_err(|e| AuthorizationError::ConnectionError { msg: e.to_string() })?;

        let channel = endpoint
            .connect()
            .await
            .map_err(|e| AuthorizationError::ConnectionError { msg: e.to_string() })?;

        Ok(channel)
    }

    async fn permissions(
        &self,
    ) -> tokio::sync::RwLockWriteGuard<
        '_,
        PermissionsServiceClient<InterceptedService<Channel, AuthInterceptor>>,
    > {
        self.permissions.write().await
    }

    /// Checks if a subject has a specific permission on a resource.
    ///
    /// This is the primary method for performing authorization checks in your application.
    /// It evaluates whether the given subject (e.g., a user) has the specified permission
    /// on the target resource (e.g., a channel, server, or other object).
    ///
    /// # How It Works
    ///
    /// The method performs the following steps:
    /// 1. Converts the resource and subject into SpiceDB object references
    /// 2. Sends a gRPC `CheckPermission` request to SpiceDB
    /// 3. SpiceDB evaluates the permission based on defined relationships and rules
    /// 4. Returns an `AuthorizationResult` indicating whether access is granted
    ///
    /// # Arguments
    ///
    /// * `resource` - The resource being accessed (e.g., `SpiceDbObject::Channel("123")`)
    /// * `permission` - The permission being checked (e.g., `Permissions::ViewChannels`)
    /// * `subject` - The entity requesting access (e.g., `SpiceDbObject::User("456")`)
    ///
    /// # Returns
    ///
    /// Returns an `AuthorizationResult` which can be queried with:
    /// - `has_permissions()` - Returns `true` if access is granted
    /// - `result()` - Returns `Ok(())` if granted, or `Err(AuthorizationError)` if denied
    ///
    /// # Examples
    ///
    /// ## Basic permission check
    ///
    /// ```no_run
    /// use authz::{SpiceDbRepository, SpiceDbObject, Permissions};
    ///
    /// # async fn example(repo: SpiceDbRepository) {
    /// let result = repo.check_permissions(
    ///     SpiceDbObject::Channel("general".to_string()),
    ///     Permissions::SendMessages,
    ///     SpiceDbObject::User("alice".to_string()),
    /// ).await;
    ///
    /// if result.has_permissions() {
    ///     // Allow user to send message
    ///     println!("User can send messages");
    /// } else {
    ///     // Deny access
    ///     println!("User cannot send messages");
    /// }
    /// # }
    /// ```
    ///
    /// ## Using result() for error handling
    ///
    /// ```no_run
    /// use authz::{SpiceDbRepository, SpiceDbObject, Permissions, AuthorizationError};
    ///
    /// # async fn example(repo: SpiceDbRepository) -> Result<(), AuthorizationError> {
    /// let result = repo.check_permissions(
    ///     SpiceDbObject::Server("server-1".to_string()),
    ///     Permissions::ManageRoles,
    ///     SpiceDbObject::User("bob".to_string()),
    /// ).await;
    ///
    /// // Returns error if permission denied
    /// result.result()?;
    ///
    /// // Continue with authorized action
    /// println!("User is authorized to manage roles");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ## Checking administrative access
    ///
    /// ```no_run
    /// use authz::{SpiceDbRepository, SpiceDbObject, Permissions};
    ///
    /// # async fn example(repo: SpiceDbRepository) {
    /// let is_admin = repo.check_permissions(
    ///     SpiceDbObject::Server("my-server".to_string()),
    ///     Permissions::Administrator,
    ///     SpiceDbObject::User("charlie".to_string()),
    /// ).await.has_permissions();
    ///
    /// if is_admin {
    ///     // Grant full access
    /// }
    /// # }
    /// ```
    ///
    /// # Performance Considerations
    ///
    /// Each call to this method makes a network request to SpiceDB. For high-performance
    /// scenarios, consider:
    /// - Caching authorization results when appropriate
    /// - Batching multiple checks when possible
    /// - Using SpiceDB's consistency guarantees to balance freshness vs. performance
    ///
    /// # See Also
    ///
    /// - [`check_permissions_raw`](Self::check_permissions_raw) - Lower-level API with more control
    /// - [`Permissions`] - Available permission types
    /// - [`SpiceDbObject`] - Resource and subject types
    pub async fn check_permissions(
        &self,
        resource: impl Into<SpiceDbObject>,
        permission: Permissions,
        subject: impl Into<SpiceDbObject>,
    ) -> AuthorizationResult {
        let resource: SpiceDbObject = resource.into();
        let subject: SpiceDbObject = subject.into();
        let permission: String = permission.to_string();
        self.check_permissions_raw(resource, permission, subject)
            .await
            .into()
    }

    /// Performs a raw permission check using SpiceDB object references and string permissions.
    ///
    /// This is a lower-level API that provides more flexibility than [`check_permissions`](Self::check_permissions).
    /// It allows you to specify permissions as strings and use custom object references directly,
    /// which can be useful for:
    /// - Custom permission types not defined in the `Permissions` enum
    /// - Dynamic permission names determined at runtime
    /// - Direct integration with SpiceDB's native types
    ///
    /// # Arguments
    ///
    /// * `resource` - The resource object reference (must implement `Into<ObjectReference>`)
    /// * `permission` - The permission name as a string (e.g., "view", "edit", "admin")
    /// * `subject` - The subject object reference (must implement `Into<ObjectReference>`)
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing:
    /// - `Ok(Permissionship)` - The permission status from SpiceDB:
    ///   - `Permissionship::HasPermission` - Access is granted
    ///   - `Permissionship::NoPermission` - Access is denied
    ///   - `Permissionship::ConditionalPermission` - Access depends on additional context
    /// - `Err(AuthorizationError::Unauthorized)` - The check failed or was denied
    ///
    /// # Examples
    ///
    /// ## Custom permission check
    ///
    /// ```no_run
    /// use authz::{SpiceDbRepository, authzed::api::v1::ObjectReference};
    ///
    /// # async fn example(repo: SpiceDbRepository) -> Result<(), Box<dyn std::error::Error>> {
    /// let resource = ObjectReference {
    ///     object_type: "document".to_string(),
    ///     object_id: "doc-123".to_string(),
    /// };
    ///
    /// let subject = ObjectReference {
    ///     object_type: "user".to_string(),
    ///     object_id: "user-456".to_string(),
    /// };
    ///
    /// let permissionship = repo.check_permissions_raw(
    ///     resource,
    ///     "edit",
    ///     subject,
    /// ).await?;
    ///
    /// match permissionship {
    ///     authz::authzed::api::v1::check_permission_response::Permissionship::HasPermission => {
    ///         println!("Permission granted");
    ///     }
    ///     _ => {
    ///         println!("Permission denied");
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ## Dynamic permission names
    ///
    /// ```no_run
    /// use authz::{SpiceDbRepository, SpiceDbObject};
    ///
    /// # async fn check_dynamic_permission(
    /// #     repo: SpiceDbRepository,
    /// #     action: &str,
    /// #     resource_id: &str,
    /// #     user_id: &str,
    /// # ) -> Result<bool, Box<dyn std::error::Error>> {
    /// let permission_name = format!("can_{}", action);
    ///
    /// let result = repo.check_permissions_raw(
    ///     SpiceDbObject::Channel(resource_id.to_string()),
    ///     permission_name,
    ///     SpiceDbObject::User(user_id.to_string()),
    /// ).await?;
    ///
    /// Ok(result.has_permissions())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The gRPC connection to SpiceDB fails
    /// - The request times out
    /// - The permission check is denied (returns `AuthorizationError::Unauthorized`)
    ///
    /// # See Also
    ///
    /// - [`check_permissions`](Self::check_permissions) - Higher-level, type-safe API
    /// - [SpiceDB CheckPermission API](https://buf.build/authzed/api/docs/main:authzed.api.v1#authzed.api.v1.PermissionsService.CheckPermission)
    pub async fn check_permissions_raw(
        &self,
        resource: impl Into<ObjectReference>,
        permission: impl Into<String>,
        subject: impl Into<ObjectReference>,
    ) -> Result<Permissionship, AuthorizationError> {
        let resource: ObjectReference = resource.into();
        let sub_object_reference: ObjectReference = subject.into();
        let subject = SubjectReference {
            object: Some(sub_object_reference),
            ..Default::default()
        };
        let check_request = CheckPermissionRequest {
            resource: Some(resource),
            permission: permission.into(),
            subject: Some(subject),
            ..Default::default()
        };

        let check_response: CheckPermissionResponse = self
            .permissions()
            .await
            .check_permission(check_request)
            .await
            .map_err(|_| AuthorizationError::Unauthorized)?
            .into_inner();

        Ok(check_response.permissionship())
    }
}

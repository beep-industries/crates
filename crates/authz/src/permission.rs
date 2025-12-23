use std::fmt::Display;

use crate::{AuthorizationError, authzed::api::v1::check_permission_response::Permissionship};

/// Enumeration of available permissions in the authorization system.
///
/// This enum defines all the permission types that can be checked against resources
/// in your application. Each permission represents a specific action or capability
/// that a subject (user) can have on a resource (server, channel, etc.).
///
/// # Permission Hierarchy
///
/// Permissions are designed to work with SpiceDB's relationship-based access control.
/// Some permissions may imply others through your SpiceDB schema definition.
///
/// # Examples
///
/// ```no_run
/// use authz::{Permissions, SpiceDbRepository, SpiceDbObject};
///
/// # async fn example(repo: SpiceDbRepository) {
/// // Check if a user can send messages in a channel
/// let result = repo.check_permissions(
///     SpiceDbObject::Channel("general".to_string()),
///     Permissions::SendMessages,
///     SpiceDbObject::User("alice".to_string()),
/// ).await;
/// # }
/// ```
///
/// # Display Format
///
/// Each permission has a string representation used when communicating with SpiceDB:
/// - `Administrator` → "admin"
/// - `ManageServer` → "manage"
/// - `ViewChannels` → "view_channel"
/// - etc.
#[derive(Debug)]
pub enum Permissions {
    /// Full administrative access to all resources in a server.
    ///
    /// Administrators can perform any action on any subject (channels, webhooks, etc.)
    /// within a server. This is the highest level of permission.
    Administrator,

    /// Permission to update server settings and configuration.
    ///
    /// Allows updating a server (all CRUD operations except delete).
    ManageServer,

    /// Permission to manage roles within the server.
    ///
    /// Allows performing all CRUD operations on roles, including creating, updating,
    /// assigning, and deleting roles.
    ManageRoles,

    /// Permission to create server invitations.
    ///
    /// Allows generating invite links that new users can use to join the server.
    CreateInvitation,

    /// Permission to manage all channels in the server.
    ///
    /// Allows performing all CRUD operations on every channel, including creating,
    /// updating, and deleting channels.
    ManageChannels,

    /// Permission to manage webhooks in the server.
    ///
    /// Allows performing all CRUD operations on every webhook.
    ManageWebhooks,

    /// Permission to view channels and their contents.
    ///
    /// Allows seeing the channel and reading its messages and metadata.
    ViewChannels,

    /// Permission to send messages in a channel.
    ///
    /// Allows posting new messages to the channel.
    SendMessages,

    /// Permission to manage other users' nicknames.
    ///
    /// Allows updating the display names of other users in the server.
    ManageNicknames,

    /// Permission to change your own nickname.
    ///
    /// Allows updating your own display name in the server.
    ChangeNickname,

    /// Permission to manage messages from other users.
    ///
    /// Allows moderating, editing, or deleting messages posted by other users.
    ManageMessages,

    /// Permission to upload files and images.
    ///
    /// Allows attaching files, images, and other media to messages.
    AttachFiles,
}

impl Display for Permissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Permissions::Administrator => write!(f, "admin"),
            Permissions::ManageServer => write!(f, "manage"),
            Permissions::ManageRoles => write!(f, "manage_role"),
            Permissions::CreateInvitation => write!(f, "create_invitation"),
            Permissions::ManageChannels => write!(f, "manage_channels"),
            Permissions::ManageWebhooks => write!(f, "manage_webhooks"),
            Permissions::ViewChannels => write!(f, "view_channel"),
            Permissions::SendMessages => write!(f, "send_message"),
            Permissions::ManageNicknames => write!(f, "manage_nicknames"),
            Permissions::ChangeNickname => write!(f, "change_nickname"),
            Permissions::ManageMessages => write!(f, "manage_message"),
            Permissions::AttachFiles => write!(f, "attach_files"),
        }
    }
}

impl Permissionship {
    /// Converts the permission check result into a `Result`.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the subject has the permission (`HasPermission`)
    /// - `Err(AuthorizationError::Unauthorized)` for any other status
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use authz::authzed::api::v1::check_permission_response::Permissionship;
    ///
    /// let permission = Permissionship::HasPermission;
    /// assert!(permission.result().is_ok());
    ///
    /// let no_permission = Permissionship::NoPermission;
    /// assert!(no_permission.result().is_err());
    /// ```
    pub fn result(&self) -> Result<(), AuthorizationError> {
        match self {
            Permissionship::HasPermission => Ok(()),
            _ => Err(AuthorizationError::Unauthorized),
        }
    }

    /// Checks if the permission is granted.
    ///
    /// # Returns
    ///
    /// `true` if the subject has the permission, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use authz::authzed::api::v1::check_permission_response::Permissionship;
    ///
    /// let permission = Permissionship::HasPermission;
    /// assert!(permission.has_permissions());
    ///
    /// let no_permission = Permissionship::NoPermission;
    /// assert!(!no_permission.has_permissions());
    /// ```
    pub fn has_permissions(&self) -> bool {
        match self {
            Permissionship::HasPermission => true,
            _ => false,
        }
    }
}

/// Wrapper around a permission check result for convenient handling.
///
/// `AuthorizationResult` provides helper methods to work with permission check results,
/// making it easier to handle both successful and failed authorization attempts.
///
/// # Examples
///
/// ```no_run
/// use authz::{SpiceDbRepository, SpiceDbObject, Permissions};
///
/// # async fn example(repo: SpiceDbRepository) {
/// let result = repo.check_permissions(
///     SpiceDbObject::Channel("general".to_string()),
///     Permissions::ViewChannels,
///     SpiceDbObject::User("alice".to_string()),
/// ).await;
///
/// // Check as boolean
/// if result.has_permissions() {
///     println!("Access granted");
/// }
///
/// // Or use as Result for error propagation
/// if let Err(e) = result.result() {
///     println!("Access denied: {}", e);
/// }
/// # }
/// ```
pub struct AuthorizationResult(Result<Permissionship, AuthorizationError>);

impl AuthorizationResult {
    /// Checks if the authorization check resulted in granted permission.
    ///
    /// This method returns `true` only if the permission check succeeded and
    /// the subject has the requested permission.
    ///
    /// # Returns
    ///
    /// `true` if permission is granted, `false` if denied or if an error occurred.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use authz::{SpiceDbRepository, SpiceDbObject, Permissions};
    ///
    /// # async fn example(repo: SpiceDbRepository) {
    /// let result = repo.check_permissions(
    ///     SpiceDbObject::Server("my-server".to_string()),
    ///     Permissions::Administrator,
    ///     SpiceDbObject::User("alice".to_string()),
    /// ).await;
    ///
    /// if result.has_permissions() {
    ///     // User is an administrator
    ///     println!("Admin access granted");
    /// } else {
    ///     // User is not an administrator or check failed
    ///     println!("Admin access denied");
    /// }
    /// # }
    /// ```
    pub fn has_permissions(&self) -> bool {
        match self.0 {
            Ok(permission) => permission.has_permissions(),
            Err(_) => false,
        }
    }

    /// Converts the authorization result into a standard `Result`.
    ///
    /// This method is useful when you want to use Rust's `?` operator for
    /// error propagation or when you need to handle authorization failures
    /// as errors in your application flow.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if permission is granted
    /// - `Err(AuthorizationError::Unauthorized)` if permission is denied or check failed
    ///
    /// # Examples
    ///
    /// ## Using with the `?` operator
    ///
    /// ```no_run
    /// use authz::{SpiceDbRepository, SpiceDbObject, Permissions, AuthorizationError};
    ///
    /// # async fn handle_request(repo: SpiceDbRepository) -> Result<(), AuthorizationError> {
    /// // Check permission and propagate error if denied
    /// repo.check_permissions(
    ///     SpiceDbObject::Channel("private".to_string()),
    ///     Permissions::ViewChannels,
    ///     SpiceDbObject::User("bob".to_string()),
    /// ).await.result()?;
    ///
    /// // This code only runs if permission was granted
    /// println!("User authorized to view channel");
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ## Explicit error handling
    ///
    /// ```no_run
    /// use authz::{SpiceDbRepository, SpiceDbObject, Permissions};
    ///
    /// # async fn example(repo: SpiceDbRepository) {
    /// let result = repo.check_permissions(
    ///     SpiceDbObject::Server("server-1".to_string()),
    ///     Permissions::ManageServer,
    ///     SpiceDbObject::User("charlie".to_string()),
    /// ).await;
    ///
    /// match result.result() {
    ///     Ok(_) => println!("User can manage server"),
    ///     Err(e) => println!("Access denied: {}", e),
    /// }
    /// # }
    /// ```
    pub fn result(&self) -> Result<(), AuthorizationError> {
        let permissions = match self.0 {
            Ok(permission) => permission,
            Err(_) => return Err(AuthorizationError::Unauthorized),
        };
        if permissions.has_permissions() {
            Ok(())
        } else {
            Err(AuthorizationError::Unauthorized)
        }
    }
}

impl From<Result<Permissionship, AuthorizationError>> for AuthorizationResult {
    fn from(value: Result<Permissionship, AuthorizationError>) -> Self {
        Self(value)
    }
}

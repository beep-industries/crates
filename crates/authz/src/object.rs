use crate::authzed::api::v1::ObjectReference;

type ObjectId = String;

/// Represents different types of objects in the SpiceDB authorization system.
///
/// `SpiceDbObject` is used to identify resources and subjects when performing
/// permission checks. Each variant represents a different type of entity in your
/// application's domain model.
///
/// # Object Types
///
/// - **Server** - A server/workspace that contains channels, users, and roles
/// - **Channel** - A communication channel within a server
/// - **User** - A user/subject that can have permissions
/// - **PermissionOverride** - A permission override rule
///
/// # Examples
///
/// ```no_run
/// use authz::{SpiceDbObject, SpiceDbRepository, Permissions};
///
/// # async fn example(repo: SpiceDbRepository) {
/// // Check if a user can view a channel
/// let result = repo.check_permissions(
///     SpiceDbObject::Channel("general-chat".to_string()),
///     Permissions::ViewChannels,
///     SpiceDbObject::User("user-123".to_string()),
/// ).await;
///
/// // Check if a user is a server admin
/// let is_admin = repo.check_permissions(
///     SpiceDbObject::Server("my-server".to_string()),
///     Permissions::Administrator,
///     SpiceDbObject::User("user-456".to_string()),
/// ).await.has_permissions();
/// # }
/// ```
///
/// # SpiceDB Integration
///
/// Each `SpiceDbObject` is converted into a SpiceDB `ObjectReference` when
/// communicating with the SpiceDB API. The object type determines the namespace
/// used in SpiceDB's schema.
pub enum SpiceDbObject {
    /// A server object identified by its unique ID.
    ///
    /// Servers are top-level containers that can have channels, users, roles,
    /// and other resources. They correspond to the "server" object type in SpiceDB.
    ///
    /// # Example
    ///
    /// ```
    /// use authz::SpiceDbObject;
    ///
    /// let server = SpiceDbObject::Server("server-abc-123".to_string());
    /// ```
    Server(ObjectId),

    /// A channel object identified by its unique ID.
    ///
    /// Channels are communication spaces within a server where users can view
    /// and send messages. They correspond to the "channel" object type in SpiceDB.
    ///
    /// # Example
    ///
    /// ```
    /// use authz::SpiceDbObject;
    ///
    /// let channel = SpiceDbObject::Channel("channel-xyz-789".to_string());
    /// ```
    Channel(ObjectId),

    /// A user object identified by its unique ID.
    ///
    /// Users are subjects that can have permissions on resources. They correspond
    /// to the "user" object type in SpiceDB.
    ///
    /// # Example
    ///
    /// ```
    /// use authz::SpiceDbObject;
    ///
    /// let user = SpiceDbObject::User("user-def-456".to_string());
    /// ```
    User(ObjectId),

    /// A permission override object identified by its unique ID.
    ///
    /// Permission overrides allow fine-grained control over access rules.
    /// They correspond to the "permission_override" object type in SpiceDB.
    ///
    /// # Example
    ///
    /// ```
    /// use authz::SpiceDbObject;
    ///
    /// let override_rule = SpiceDbObject::PermissionOverride("override-001".to_string());
    /// ```
    PermissionOverride(ObjectId),
}

impl SpiceDbObject {
    /// Returns the object's unique identifier.
    ///
    /// Extracts the ID string from the object, regardless of its type.
    pub(crate) fn id(&self) -> ObjectId {
        match self {
            SpiceDbObject::Server(id) => id.clone(),
            SpiceDbObject::Channel(id) => id.clone(),
            SpiceDbObject::User(id) => id.clone(),
            SpiceDbObject::PermissionOverride(id) => id.clone(),
        }
    }

    /// Returns the SpiceDB object type name.
    ///
    /// This corresponds to the object type namespace defined in your SpiceDB schema.
    ///
    /// # Returns
    ///
    /// - `"server"` for `SpiceDbObject::Server`
    /// - `"channel"` for `SpiceDbObject::Channel`
    /// - `"user"` for `SpiceDbObject::User`
    /// - `"permission_override"` for `SpiceDbObject::PermissionOverride`
    pub(crate) fn object_name(&self) -> String {
        match self {
            SpiceDbObject::Server(_) => "server".to_string(),
            SpiceDbObject::Channel(_) => "channel".to_string(),
            SpiceDbObject::User(_) => "user".to_string(),
            SpiceDbObject::PermissionOverride(_) => "permission_override".to_string(),
        }
    }
}

/// Converts a `SpiceDbObject` into a SpiceDB `ObjectReference`.
///
/// This implementation allows `SpiceDbObject` to be used directly in permission
/// check operations. The conversion maps the object to SpiceDB's wire format.
impl Into<ObjectReference> for SpiceDbObject {
    fn into(self) -> ObjectReference {
        ObjectReference {
            object_type: self.id(),
            object_id: self.object_name(),
        }
    }
}

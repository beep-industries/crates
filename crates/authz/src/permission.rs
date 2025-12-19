use std::fmt::Display;

use crate::{AuthorizationError, authzed::api::v1::check_permission_response::Permissionship};

#[derive(Debug)]
pub enum Permissions {
    Administrator,    // Can do any action on any subject (channel, webhooks…) in a server.
    ManageServer,     // Can update a server (all CRUD except delete).
    ManageRoles,      // Can do all CRUD operations on all roles.
    CreateInvitation, // Can create server invites.
    ManageChannels,   // Can do all CRUD operations on every channel.
    ManageWebhooks,   // Can do all CRUD operations on every webhook.
    ViewChannels,     // Can see the channel and its contents (messages).
    SendMessages,     // Can send a message on the channel.
    ManageNicknames,  // Can update other users' nicknames.
    ChangeNickname,   // Can update your own nickname.
    ManageMessages,   // Can delete other users' messages.
    AttachFiles,      // Can upload images and files.
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
    pub fn result(&self) -> Result<(), AuthorizationError> {
        match self {
            Permissionship::HasPermission => Ok(()),
            _ => Err(AuthorizationError::Unauthorized),
        }
    }
    pub fn has_permissions(&self) -> bool {
        match self {
            Permissionship::HasPermission => true,
            _ => false,
        }
    }
}

pub struct AuthorizationResult(Result<Permissionship, AuthorizationError>);

impl AuthorizationResult {
    pub fn has_permissions(&self) -> bool {
        match self.0 {
            Ok(permission) => permission.has_permissions(),
            Err(_) => false,
        }
    }

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

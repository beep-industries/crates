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

// Main AuthZed client with all service clients
#[derive(Clone)]
pub struct SpiceDbRepository {
    permissions:
        Arc<RwLock<PermissionsServiceClient<InterceptedService<Channel, AuthInterceptor>>>>,
}

impl SpiceDbRepository {
    /// Create a new SpiceDb client with the given configuration

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

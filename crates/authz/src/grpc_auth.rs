use tonic::service::Interceptor;

// Interceptor for adding authentication token to requests
#[derive(Clone)]
pub(crate) struct AuthInterceptor {
    token: String,
}

impl AuthInterceptor {
    pub fn new(token: String) -> Self {
        Self { token }
    }
}

impl Interceptor for AuthInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, tonic::Status> {
        // Only add auth header if token is not empty
        if !self.token.is_empty() {
            let token = format!("Bearer {}", self.token);
            let metadata_value = tonic::metadata::MetadataValue::try_from(token)
                .map_err(|e| tonic::Status::internal(format!("Invalid token: {}", e)))?;
            request
                .metadata_mut()
                .insert("authorization", metadata_value);
        }
        Ok(request)
    }
}

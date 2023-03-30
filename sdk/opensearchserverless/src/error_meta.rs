// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>When creating a collection, thrown when a collection with the same name already exists or is being created. When deleting a collection, thrown when the collection is not in the ACTIVE or FAILED state.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>Thrown when an error internal to the service occurs while processing a request.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>Thrown when accessing or deleting a resource that does not exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>Thrown when the HTTP request contains invalid input or is missing required input.</p>
    ValidationException(crate::error::ValidationException),
    /// 
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    /// 
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    /// 
    Unhandled(crate::error::Unhandled)
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::BatchGetCollectionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::BatchGetCollectionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::BatchGetCollectionError> for Error {
    fn from(err: crate::error::BatchGetCollectionError) -> Self {
        match err.kind {
            crate::error::BatchGetCollectionErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::BatchGetCollectionErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::BatchGetCollectionErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::BatchGetVpcEndpointError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::BatchGetVpcEndpointError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::BatchGetVpcEndpointError> for Error {
    fn from(err: crate::error::BatchGetVpcEndpointError) -> Self {
        match err.kind {
            crate::error::BatchGetVpcEndpointErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::BatchGetVpcEndpointErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::BatchGetVpcEndpointErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateAccessPolicyError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateAccessPolicyError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateAccessPolicyError> for Error {
    fn from(err: crate::error::CreateAccessPolicyError) -> Self {
        match err.kind {
            crate::error::CreateAccessPolicyErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::CreateAccessPolicyErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::CreateAccessPolicyErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::CreateAccessPolicyErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateCollectionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateCollectionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateCollectionError> for Error {
    fn from(err: crate::error::CreateCollectionError) -> Self {
        match err.kind {
            crate::error::CreateCollectionErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::CreateCollectionErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::CreateCollectionErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::CreateCollectionErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateSecurityConfigError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateSecurityConfigError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateSecurityConfigError> for Error {
    fn from(err: crate::error::CreateSecurityConfigError) -> Self {
        match err.kind {
            crate::error::CreateSecurityConfigErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::CreateSecurityConfigErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::CreateSecurityConfigErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::CreateSecurityConfigErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateSecurityPolicyError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateSecurityPolicyError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateSecurityPolicyError> for Error {
    fn from(err: crate::error::CreateSecurityPolicyError) -> Self {
        match err.kind {
            crate::error::CreateSecurityPolicyErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::CreateSecurityPolicyErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::CreateSecurityPolicyErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::CreateSecurityPolicyErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateVpcEndpointError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateVpcEndpointError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateVpcEndpointError> for Error {
    fn from(err: crate::error::CreateVpcEndpointError) -> Self {
        match err.kind {
            crate::error::CreateVpcEndpointErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::CreateVpcEndpointErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::CreateVpcEndpointErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::CreateVpcEndpointErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteAccessPolicyError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteAccessPolicyError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteAccessPolicyError> for Error {
    fn from(err: crate::error::DeleteAccessPolicyError) -> Self {
        match err.kind {
            crate::error::DeleteAccessPolicyErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::DeleteAccessPolicyErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::DeleteAccessPolicyErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeleteAccessPolicyErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::DeleteAccessPolicyErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteCollectionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteCollectionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteCollectionError> for Error {
    fn from(err: crate::error::DeleteCollectionError) -> Self {
        match err.kind {
            crate::error::DeleteCollectionErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::DeleteCollectionErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::DeleteCollectionErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeleteCollectionErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::DeleteCollectionErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteSecurityConfigError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteSecurityConfigError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteSecurityConfigError> for Error {
    fn from(err: crate::error::DeleteSecurityConfigError) -> Self {
        match err.kind {
            crate::error::DeleteSecurityConfigErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::DeleteSecurityConfigErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::DeleteSecurityConfigErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeleteSecurityConfigErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::DeleteSecurityConfigErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteSecurityPolicyError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteSecurityPolicyError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteSecurityPolicyError> for Error {
    fn from(err: crate::error::DeleteSecurityPolicyError) -> Self {
        match err.kind {
            crate::error::DeleteSecurityPolicyErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::DeleteSecurityPolicyErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::DeleteSecurityPolicyErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeleteSecurityPolicyErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::DeleteSecurityPolicyErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteVpcEndpointError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteVpcEndpointError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteVpcEndpointError> for Error {
    fn from(err: crate::error::DeleteVpcEndpointError) -> Self {
        match err.kind {
            crate::error::DeleteVpcEndpointErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::DeleteVpcEndpointErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::DeleteVpcEndpointErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeleteVpcEndpointErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::DeleteVpcEndpointErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetAccessPolicyError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetAccessPolicyError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetAccessPolicyError> for Error {
    fn from(err: crate::error::GetAccessPolicyError) -> Self {
        match err.kind {
            crate::error::GetAccessPolicyErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::GetAccessPolicyErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetAccessPolicyErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::GetAccessPolicyErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetAccountSettingsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetAccountSettingsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetAccountSettingsError> for Error {
    fn from(err: crate::error::GetAccountSettingsError) -> Self {
        match err.kind {
            crate::error::GetAccountSettingsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::GetAccountSettingsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::GetAccountSettingsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetPoliciesStatsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetPoliciesStatsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetPoliciesStatsError> for Error {
    fn from(err: crate::error::GetPoliciesStatsError) -> Self {
        match err.kind {
            crate::error::GetPoliciesStatsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::GetPoliciesStatsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetSecurityConfigError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetSecurityConfigError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetSecurityConfigError> for Error {
    fn from(err: crate::error::GetSecurityConfigError) -> Self {
        match err.kind {
            crate::error::GetSecurityConfigErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::GetSecurityConfigErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetSecurityConfigErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::GetSecurityConfigErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetSecurityPolicyError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetSecurityPolicyError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetSecurityPolicyError> for Error {
    fn from(err: crate::error::GetSecurityPolicyError) -> Self {
        match err.kind {
            crate::error::GetSecurityPolicyErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::GetSecurityPolicyErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetSecurityPolicyErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::GetSecurityPolicyErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListAccessPoliciesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListAccessPoliciesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListAccessPoliciesError> for Error {
    fn from(err: crate::error::ListAccessPoliciesError) -> Self {
        match err.kind {
            crate::error::ListAccessPoliciesErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::ListAccessPoliciesErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListAccessPoliciesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListCollectionsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListCollectionsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListCollectionsError> for Error {
    fn from(err: crate::error::ListCollectionsError) -> Self {
        match err.kind {
            crate::error::ListCollectionsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::ListCollectionsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListCollectionsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListSecurityConfigsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListSecurityConfigsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListSecurityConfigsError> for Error {
    fn from(err: crate::error::ListSecurityConfigsError) -> Self {
        match err.kind {
            crate::error::ListSecurityConfigsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::ListSecurityConfigsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListSecurityConfigsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListSecurityPoliciesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListSecurityPoliciesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListSecurityPoliciesError> for Error {
    fn from(err: crate::error::ListSecurityPoliciesError) -> Self {
        match err.kind {
            crate::error::ListSecurityPoliciesErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::ListSecurityPoliciesErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListSecurityPoliciesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListTagsForResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListTagsForResourceError> for Error {
    fn from(err: crate::error::ListTagsForResourceError) -> Self {
        match err.kind {
            crate::error::ListTagsForResourceErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::ListTagsForResourceErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListVpcEndpointsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListVpcEndpointsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListVpcEndpointsError> for Error {
    fn from(err: crate::error::ListVpcEndpointsError) -> Self {
        match err.kind {
            crate::error::ListVpcEndpointsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::ListVpcEndpointsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListVpcEndpointsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::TagResourceError> for Error {
    fn from(err: crate::error::TagResourceError) -> Self {
        match err.kind {
            crate::error::TagResourceErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::TagResourceErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::TagResourceErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::TagResourceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UntagResourceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UntagResourceError> for Error {
    fn from(err: crate::error::UntagResourceError) -> Self {
        match err.kind {
            crate::error::UntagResourceErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::UntagResourceErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UntagResourceErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateAccessPolicyError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateAccessPolicyError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateAccessPolicyError> for Error {
    fn from(err: crate::error::UpdateAccessPolicyError) -> Self {
        match err.kind {
            crate::error::UpdateAccessPolicyErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::UpdateAccessPolicyErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::UpdateAccessPolicyErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UpdateAccessPolicyErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::UpdateAccessPolicyErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateAccountSettingsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateAccountSettingsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateAccountSettingsError> for Error {
    fn from(err: crate::error::UpdateAccountSettingsError) -> Self {
        match err.kind {
            crate::error::UpdateAccountSettingsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::UpdateAccountSettingsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::UpdateAccountSettingsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateCollectionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateCollectionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateCollectionError> for Error {
    fn from(err: crate::error::UpdateCollectionError) -> Self {
        match err.kind {
            crate::error::UpdateCollectionErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::UpdateCollectionErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::UpdateCollectionErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::UpdateCollectionErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateSecurityConfigError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateSecurityConfigError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateSecurityConfigError> for Error {
    fn from(err: crate::error::UpdateSecurityConfigError) -> Self {
        match err.kind {
            crate::error::UpdateSecurityConfigErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::UpdateSecurityConfigErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::UpdateSecurityConfigErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UpdateSecurityConfigErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::UpdateSecurityConfigErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateSecurityPolicyError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateSecurityPolicyError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateSecurityPolicyError> for Error {
    fn from(err: crate::error::UpdateSecurityPolicyError) -> Self {
        match err.kind {
            crate::error::UpdateSecurityPolicyErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::UpdateSecurityPolicyErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::UpdateSecurityPolicyErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UpdateSecurityPolicyErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::UpdateSecurityPolicyErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateVpcEndpointError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateVpcEndpointError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateVpcEndpointError> for Error {
    fn from(err: crate::error::UpdateVpcEndpointError) -> Self {
        match err.kind {
            crate::error::UpdateVpcEndpointErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::UpdateVpcEndpointErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::UpdateVpcEndpointErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::UpdateVpcEndpointErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl std::error::Error for Error {}


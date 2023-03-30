// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>The health check for the instance that's specified by <code>ServiceId</code> and <code>InstanceId</code> isn't a custom health check. </p>
    CustomHealthNotFound(crate::error::CustomHealthNotFound),
    /// <p>The operation is already in progress.</p>
    DuplicateRequest(crate::error::DuplicateRequest),
    /// <p>No instance exists with the specified ID, or the instance was recently registered, and information about the instance hasn't propagated yet.</p>
    InstanceNotFound(crate::error::InstanceNotFound),
    /// <p>One or more specified values aren't valid. For example, a required value might be missing, a numeric value might be outside the allowed range, or a string value might exceed length constraints.</p>
    InvalidInput(crate::error::InvalidInput),
    /// <p>The namespace that you're trying to create already exists.</p>
    NamespaceAlreadyExists(crate::error::NamespaceAlreadyExists),
    /// <p>No namespace exists with the specified ID.</p>
    NamespaceNotFound(crate::error::NamespaceNotFound),
    /// <p>No operation exists with the specified ID.</p>
    OperationNotFound(crate::error::OperationNotFound),
    /// <p>The operation can't be completed because you've reached the quota for the number of requests. For more information, see <a href="https://docs.aws.amazon.com/cloud-map/latest/dg/throttling.html">Cloud Map API request throttling quota</a> in the <i>Cloud Map Developer Guide</i>.</p>
    RequestLimitExceeded(crate::error::RequestLimitExceeded),
    /// <p>The specified resource can't be deleted because it contains other resources. For example, you can't delete a service that contains any instances.</p>
    ResourceInUse(crate::error::ResourceInUse),
    /// <p>The resource can't be created because you've reached the quota on the number of resources.</p>
    ResourceLimitExceeded(crate::error::ResourceLimitExceeded),
    /// <p>The operation can't be completed because the resource was not found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The service can't be created because a service with the same name already exists.</p>
    ServiceAlreadyExists(crate::error::ServiceAlreadyExists),
    /// <p>No service exists with the specified ID.</p>
    ServiceNotFound(crate::error::ServiceNotFound),
    /// <p>The list of tags on the resource is over the quota. The maximum number of tags that can be applied to a resource is 50.</p>
    TooManyTagsException(crate::error::TooManyTagsException),
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
            Error::CustomHealthNotFound(inner) => inner.fmt(f),
            Error::DuplicateRequest(inner) => inner.fmt(f),
            Error::InstanceNotFound(inner) => inner.fmt(f),
            Error::InvalidInput(inner) => inner.fmt(f),
            Error::NamespaceAlreadyExists(inner) => inner.fmt(f),
            Error::NamespaceNotFound(inner) => inner.fmt(f),
            Error::OperationNotFound(inner) => inner.fmt(f),
            Error::RequestLimitExceeded(inner) => inner.fmt(f),
            Error::ResourceInUse(inner) => inner.fmt(f),
            Error::ResourceLimitExceeded(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceAlreadyExists(inner) => inner.fmt(f),
            Error::ServiceNotFound(inner) => inner.fmt(f),
            Error::TooManyTagsException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateHttpNamespaceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateHttpNamespaceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateHttpNamespaceError> for Error {
    fn from(err: crate::error::CreateHttpNamespaceError) -> Self {
        match err.kind {
            crate::error::CreateHttpNamespaceErrorKind::DuplicateRequest(inner) => Error::DuplicateRequest(inner),
            crate::error::CreateHttpNamespaceErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::CreateHttpNamespaceErrorKind::NamespaceAlreadyExists(inner) => Error::NamespaceAlreadyExists(inner),
            crate::error::CreateHttpNamespaceErrorKind::ResourceLimitExceeded(inner) => Error::ResourceLimitExceeded(inner),
            crate::error::CreateHttpNamespaceErrorKind::TooManyTagsException(inner) => Error::TooManyTagsException(inner),
            crate::error::CreateHttpNamespaceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreatePrivateDnsNamespaceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreatePrivateDnsNamespaceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreatePrivateDnsNamespaceError> for Error {
    fn from(err: crate::error::CreatePrivateDnsNamespaceError) -> Self {
        match err.kind {
            crate::error::CreatePrivateDnsNamespaceErrorKind::DuplicateRequest(inner) => Error::DuplicateRequest(inner),
            crate::error::CreatePrivateDnsNamespaceErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::CreatePrivateDnsNamespaceErrorKind::NamespaceAlreadyExists(inner) => Error::NamespaceAlreadyExists(inner),
            crate::error::CreatePrivateDnsNamespaceErrorKind::ResourceLimitExceeded(inner) => Error::ResourceLimitExceeded(inner),
            crate::error::CreatePrivateDnsNamespaceErrorKind::TooManyTagsException(inner) => Error::TooManyTagsException(inner),
            crate::error::CreatePrivateDnsNamespaceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreatePublicDnsNamespaceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreatePublicDnsNamespaceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreatePublicDnsNamespaceError> for Error {
    fn from(err: crate::error::CreatePublicDnsNamespaceError) -> Self {
        match err.kind {
            crate::error::CreatePublicDnsNamespaceErrorKind::DuplicateRequest(inner) => Error::DuplicateRequest(inner),
            crate::error::CreatePublicDnsNamespaceErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::CreatePublicDnsNamespaceErrorKind::NamespaceAlreadyExists(inner) => Error::NamespaceAlreadyExists(inner),
            crate::error::CreatePublicDnsNamespaceErrorKind::ResourceLimitExceeded(inner) => Error::ResourceLimitExceeded(inner),
            crate::error::CreatePublicDnsNamespaceErrorKind::TooManyTagsException(inner) => Error::TooManyTagsException(inner),
            crate::error::CreatePublicDnsNamespaceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateServiceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateServiceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateServiceError> for Error {
    fn from(err: crate::error::CreateServiceError) -> Self {
        match err.kind {
            crate::error::CreateServiceErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::CreateServiceErrorKind::NamespaceNotFound(inner) => Error::NamespaceNotFound(inner),
            crate::error::CreateServiceErrorKind::ResourceLimitExceeded(inner) => Error::ResourceLimitExceeded(inner),
            crate::error::CreateServiceErrorKind::ServiceAlreadyExists(inner) => Error::ServiceAlreadyExists(inner),
            crate::error::CreateServiceErrorKind::TooManyTagsException(inner) => Error::TooManyTagsException(inner),
            crate::error::CreateServiceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteNamespaceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteNamespaceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteNamespaceError> for Error {
    fn from(err: crate::error::DeleteNamespaceError) -> Self {
        match err.kind {
            crate::error::DeleteNamespaceErrorKind::DuplicateRequest(inner) => Error::DuplicateRequest(inner),
            crate::error::DeleteNamespaceErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::DeleteNamespaceErrorKind::NamespaceNotFound(inner) => Error::NamespaceNotFound(inner),
            crate::error::DeleteNamespaceErrorKind::ResourceInUse(inner) => Error::ResourceInUse(inner),
            crate::error::DeleteNamespaceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteServiceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteServiceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteServiceError> for Error {
    fn from(err: crate::error::DeleteServiceError) -> Self {
        match err.kind {
            crate::error::DeleteServiceErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::DeleteServiceErrorKind::ResourceInUse(inner) => Error::ResourceInUse(inner),
            crate::error::DeleteServiceErrorKind::ServiceNotFound(inner) => Error::ServiceNotFound(inner),
            crate::error::DeleteServiceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeregisterInstanceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeregisterInstanceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeregisterInstanceError> for Error {
    fn from(err: crate::error::DeregisterInstanceError) -> Self {
        match err.kind {
            crate::error::DeregisterInstanceErrorKind::DuplicateRequest(inner) => Error::DuplicateRequest(inner),
            crate::error::DeregisterInstanceErrorKind::InstanceNotFound(inner) => Error::InstanceNotFound(inner),
            crate::error::DeregisterInstanceErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::DeregisterInstanceErrorKind::ResourceInUse(inner) => Error::ResourceInUse(inner),
            crate::error::DeregisterInstanceErrorKind::ServiceNotFound(inner) => Error::ServiceNotFound(inner),
            crate::error::DeregisterInstanceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DiscoverInstancesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DiscoverInstancesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DiscoverInstancesError> for Error {
    fn from(err: crate::error::DiscoverInstancesError) -> Self {
        match err.kind {
            crate::error::DiscoverInstancesErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::DiscoverInstancesErrorKind::NamespaceNotFound(inner) => Error::NamespaceNotFound(inner),
            crate::error::DiscoverInstancesErrorKind::RequestLimitExceeded(inner) => Error::RequestLimitExceeded(inner),
            crate::error::DiscoverInstancesErrorKind::ServiceNotFound(inner) => Error::ServiceNotFound(inner),
            crate::error::DiscoverInstancesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetInstanceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetInstanceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetInstanceError> for Error {
    fn from(err: crate::error::GetInstanceError) -> Self {
        match err.kind {
            crate::error::GetInstanceErrorKind::InstanceNotFound(inner) => Error::InstanceNotFound(inner),
            crate::error::GetInstanceErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::GetInstanceErrorKind::ServiceNotFound(inner) => Error::ServiceNotFound(inner),
            crate::error::GetInstanceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetInstancesHealthStatusError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetInstancesHealthStatusError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetInstancesHealthStatusError> for Error {
    fn from(err: crate::error::GetInstancesHealthStatusError) -> Self {
        match err.kind {
            crate::error::GetInstancesHealthStatusErrorKind::InstanceNotFound(inner) => Error::InstanceNotFound(inner),
            crate::error::GetInstancesHealthStatusErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::GetInstancesHealthStatusErrorKind::ServiceNotFound(inner) => Error::ServiceNotFound(inner),
            crate::error::GetInstancesHealthStatusErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetNamespaceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetNamespaceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetNamespaceError> for Error {
    fn from(err: crate::error::GetNamespaceError) -> Self {
        match err.kind {
            crate::error::GetNamespaceErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::GetNamespaceErrorKind::NamespaceNotFound(inner) => Error::NamespaceNotFound(inner),
            crate::error::GetNamespaceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetOperationError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetOperationError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetOperationError> for Error {
    fn from(err: crate::error::GetOperationError) -> Self {
        match err.kind {
            crate::error::GetOperationErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::GetOperationErrorKind::OperationNotFound(inner) => Error::OperationNotFound(inner),
            crate::error::GetOperationErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetServiceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetServiceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetServiceError> for Error {
    fn from(err: crate::error::GetServiceError) -> Self {
        match err.kind {
            crate::error::GetServiceErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::GetServiceErrorKind::ServiceNotFound(inner) => Error::ServiceNotFound(inner),
            crate::error::GetServiceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListInstancesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListInstancesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListInstancesError> for Error {
    fn from(err: crate::error::ListInstancesError) -> Self {
        match err.kind {
            crate::error::ListInstancesErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::ListInstancesErrorKind::ServiceNotFound(inner) => Error::ServiceNotFound(inner),
            crate::error::ListInstancesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListNamespacesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListNamespacesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListNamespacesError> for Error {
    fn from(err: crate::error::ListNamespacesError) -> Self {
        match err.kind {
            crate::error::ListNamespacesErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::ListNamespacesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListOperationsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListOperationsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListOperationsError> for Error {
    fn from(err: crate::error::ListOperationsError) -> Self {
        match err.kind {
            crate::error::ListOperationsErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::ListOperationsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListServicesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListServicesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListServicesError> for Error {
    fn from(err: crate::error::ListServicesError) -> Self {
        match err.kind {
            crate::error::ListServicesErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::ListServicesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
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
            crate::error::ListTagsForResourceErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RegisterInstanceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::RegisterInstanceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::RegisterInstanceError> for Error {
    fn from(err: crate::error::RegisterInstanceError) -> Self {
        match err.kind {
            crate::error::RegisterInstanceErrorKind::DuplicateRequest(inner) => Error::DuplicateRequest(inner),
            crate::error::RegisterInstanceErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::RegisterInstanceErrorKind::ResourceInUse(inner) => Error::ResourceInUse(inner),
            crate::error::RegisterInstanceErrorKind::ResourceLimitExceeded(inner) => Error::ResourceLimitExceeded(inner),
            crate::error::RegisterInstanceErrorKind::ServiceNotFound(inner) => Error::ServiceNotFound(inner),
            crate::error::RegisterInstanceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
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
            crate::error::TagResourceErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::TagResourceErrorKind::TooManyTagsException(inner) => Error::TooManyTagsException(inner),
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
            crate::error::UntagResourceErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateHttpNamespaceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateHttpNamespaceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateHttpNamespaceError> for Error {
    fn from(err: crate::error::UpdateHttpNamespaceError) -> Self {
        match err.kind {
            crate::error::UpdateHttpNamespaceErrorKind::DuplicateRequest(inner) => Error::DuplicateRequest(inner),
            crate::error::UpdateHttpNamespaceErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::UpdateHttpNamespaceErrorKind::NamespaceNotFound(inner) => Error::NamespaceNotFound(inner),
            crate::error::UpdateHttpNamespaceErrorKind::ResourceInUse(inner) => Error::ResourceInUse(inner),
            crate::error::UpdateHttpNamespaceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateInstanceCustomHealthStatusError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateInstanceCustomHealthStatusError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateInstanceCustomHealthStatusError> for Error {
    fn from(err: crate::error::UpdateInstanceCustomHealthStatusError) -> Self {
        match err.kind {
            crate::error::UpdateInstanceCustomHealthStatusErrorKind::CustomHealthNotFound(inner) => Error::CustomHealthNotFound(inner),
            crate::error::UpdateInstanceCustomHealthStatusErrorKind::InstanceNotFound(inner) => Error::InstanceNotFound(inner),
            crate::error::UpdateInstanceCustomHealthStatusErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::UpdateInstanceCustomHealthStatusErrorKind::ServiceNotFound(inner) => Error::ServiceNotFound(inner),
            crate::error::UpdateInstanceCustomHealthStatusErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdatePrivateDnsNamespaceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdatePrivateDnsNamespaceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdatePrivateDnsNamespaceError> for Error {
    fn from(err: crate::error::UpdatePrivateDnsNamespaceError) -> Self {
        match err.kind {
            crate::error::UpdatePrivateDnsNamespaceErrorKind::DuplicateRequest(inner) => Error::DuplicateRequest(inner),
            crate::error::UpdatePrivateDnsNamespaceErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::UpdatePrivateDnsNamespaceErrorKind::NamespaceNotFound(inner) => Error::NamespaceNotFound(inner),
            crate::error::UpdatePrivateDnsNamespaceErrorKind::ResourceInUse(inner) => Error::ResourceInUse(inner),
            crate::error::UpdatePrivateDnsNamespaceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdatePublicDnsNamespaceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdatePublicDnsNamespaceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdatePublicDnsNamespaceError> for Error {
    fn from(err: crate::error::UpdatePublicDnsNamespaceError) -> Self {
        match err.kind {
            crate::error::UpdatePublicDnsNamespaceErrorKind::DuplicateRequest(inner) => Error::DuplicateRequest(inner),
            crate::error::UpdatePublicDnsNamespaceErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::UpdatePublicDnsNamespaceErrorKind::NamespaceNotFound(inner) => Error::NamespaceNotFound(inner),
            crate::error::UpdatePublicDnsNamespaceErrorKind::ResourceInUse(inner) => Error::ResourceInUse(inner),
            crate::error::UpdatePublicDnsNamespaceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateServiceError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateServiceError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateServiceError> for Error {
    fn from(err: crate::error::UpdateServiceError) -> Self {
        match err.kind {
            crate::error::UpdateServiceErrorKind::DuplicateRequest(inner) => Error::DuplicateRequest(inner),
            crate::error::UpdateServiceErrorKind::InvalidInput(inner) => Error::InvalidInput(inner),
            crate::error::UpdateServiceErrorKind::ServiceNotFound(inner) => Error::ServiceNotFound(inner),
            crate::error::UpdateServiceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl std::error::Error for Error {}


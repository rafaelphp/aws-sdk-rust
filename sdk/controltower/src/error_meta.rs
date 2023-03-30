// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>User does not have sufficient access to perform this action. </p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>Updating or deleting a resource can cause an inconsistent state.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>Unexpected error during processing of request.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>Request references a resource which does not exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>Request would cause a service quota to be exceeded. The limit is 10 concurrent operations. </p>
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    /// <p> Request was denied due to request throttling.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
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
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DisableControlError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DisableControlError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DisableControlError> for Error {
    fn from(err: crate::error::DisableControlError) -> Self {
        match err.kind {
            crate::error::DisableControlErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DisableControlErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::DisableControlErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::DisableControlErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DisableControlErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::error::DisableControlErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DisableControlErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::DisableControlErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::EnableControlError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::EnableControlError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::EnableControlError> for Error {
    fn from(err: crate::error::EnableControlError) -> Self {
        match err.kind {
            crate::error::EnableControlErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::EnableControlErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::EnableControlErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::EnableControlErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::EnableControlErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::error::EnableControlErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::EnableControlErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::EnableControlErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetControlOperationError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetControlOperationError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetControlOperationError> for Error {
    fn from(err: crate::error::GetControlOperationError) -> Self {
        match err.kind {
            crate::error::GetControlOperationErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::GetControlOperationErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::GetControlOperationErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetControlOperationErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::GetControlOperationErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::GetControlOperationErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListEnabledControlsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListEnabledControlsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListEnabledControlsError> for Error {
    fn from(err: crate::error::ListEnabledControlsError) -> Self {
        match err.kind {
            crate::error::ListEnabledControlsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ListEnabledControlsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::ListEnabledControlsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::ListEnabledControlsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::ListEnabledControlsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListEnabledControlsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl std::error::Error for Error {}


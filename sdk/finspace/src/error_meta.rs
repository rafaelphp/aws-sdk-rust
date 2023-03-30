// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>The request processing has failed because of an unknown error, exception or failure.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The request is invalid. Something is wrong with the input to the request.</p>
    InvalidRequestException(crate::error::InvalidRequestException),
    /// <p>A service limit or quota is exceeded.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p> You have exceeded your service quota. To perform the requested action, remove some of the relevant resources, or use Service Quotas to request a service quota increase.</p>
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    /// <p>The request was denied due to request throttling.</p>
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
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::InvalidRequestException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateEnvironmentError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateEnvironmentError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateEnvironmentError> for Error {
    fn from(err: crate::error::CreateEnvironmentError) -> Self {
        match err.kind {
            crate::error::CreateEnvironmentErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::CreateEnvironmentErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::CreateEnvironmentErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
            crate::error::CreateEnvironmentErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::error::CreateEnvironmentErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::CreateEnvironmentErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::CreateEnvironmentErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteEnvironmentError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteEnvironmentError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteEnvironmentError> for Error {
    fn from(err: crate::error::DeleteEnvironmentError) -> Self {
        match err.kind {
            crate::error::DeleteEnvironmentErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DeleteEnvironmentErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::DeleteEnvironmentErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeleteEnvironmentErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DeleteEnvironmentErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::DeleteEnvironmentErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetEnvironmentError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetEnvironmentError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetEnvironmentError> for Error {
    fn from(err: crate::error::GetEnvironmentError) -> Self {
        match err.kind {
            crate::error::GetEnvironmentErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::GetEnvironmentErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::GetEnvironmentErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetEnvironmentErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::GetEnvironmentErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListEnvironmentsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListEnvironmentsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListEnvironmentsError> for Error {
    fn from(err: crate::error::ListEnvironmentsError) -> Self {
        match err.kind {
            crate::error::ListEnvironmentsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::ListEnvironmentsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListEnvironmentsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
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
            crate::error::ListTagsForResourceErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
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
            crate::error::TagResourceErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::TagResourceErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
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
            crate::error::UntagResourceErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::UntagResourceErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateEnvironmentError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateEnvironmentError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateEnvironmentError> for Error {
    fn from(err: crate::error::UpdateEnvironmentError) -> Self {
        match err.kind {
            crate::error::UpdateEnvironmentErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::UpdateEnvironmentErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::UpdateEnvironmentErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UpdateEnvironmentErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::UpdateEnvironmentErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::UpdateEnvironmentErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl std::error::Error for Error {}


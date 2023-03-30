// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p></p>
    InternalFailureException(crate::error::InternalFailureException),
    /// <p></p>
    InvalidRequestException(crate::error::InvalidRequestException),
    /// <p></p>
    ResourceConflictException(crate::error::ResourceConflictException),
    /// <p></p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p></p>
    TooManyRequestsException(crate::error::TooManyRequestsException),
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
            Error::InternalFailureException(inner) => inner.fmt(f),
            Error::InvalidRequestException(inner) => inner.fmt(f),
            Error::ResourceConflictException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::TooManyRequestsException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AssociateDeviceWithPlacementError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::AssociateDeviceWithPlacementError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::AssociateDeviceWithPlacementError> for Error {
    fn from(err: crate::error::AssociateDeviceWithPlacementError) -> Self {
        match err.kind {
            crate::error::AssociateDeviceWithPlacementErrorKind::InternalFailureException(inner) => Error::InternalFailureException(inner),
            crate::error::AssociateDeviceWithPlacementErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::error::AssociateDeviceWithPlacementErrorKind::ResourceConflictException(inner) => Error::ResourceConflictException(inner),
            crate::error::AssociateDeviceWithPlacementErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::AssociateDeviceWithPlacementErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreatePlacementError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreatePlacementError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreatePlacementError> for Error {
    fn from(err: crate::error::CreatePlacementError) -> Self {
        match err.kind {
            crate::error::CreatePlacementErrorKind::InternalFailureException(inner) => Error::InternalFailureException(inner),
            crate::error::CreatePlacementErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::error::CreatePlacementErrorKind::ResourceConflictException(inner) => Error::ResourceConflictException(inner),
            crate::error::CreatePlacementErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::CreatePlacementErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateProjectError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateProjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateProjectError> for Error {
    fn from(err: crate::error::CreateProjectError) -> Self {
        match err.kind {
            crate::error::CreateProjectErrorKind::InternalFailureException(inner) => Error::InternalFailureException(inner),
            crate::error::CreateProjectErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::error::CreateProjectErrorKind::ResourceConflictException(inner) => Error::ResourceConflictException(inner),
            crate::error::CreateProjectErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeletePlacementError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeletePlacementError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeletePlacementError> for Error {
    fn from(err: crate::error::DeletePlacementError) -> Self {
        match err.kind {
            crate::error::DeletePlacementErrorKind::InternalFailureException(inner) => Error::InternalFailureException(inner),
            crate::error::DeletePlacementErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::error::DeletePlacementErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeletePlacementErrorKind::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
            crate::error::DeletePlacementErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteProjectError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteProjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteProjectError> for Error {
    fn from(err: crate::error::DeleteProjectError) -> Self {
        match err.kind {
            crate::error::DeleteProjectErrorKind::InternalFailureException(inner) => Error::InternalFailureException(inner),
            crate::error::DeleteProjectErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::error::DeleteProjectErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeleteProjectErrorKind::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
            crate::error::DeleteProjectErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribePlacementError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribePlacementError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribePlacementError> for Error {
    fn from(err: crate::error::DescribePlacementError) -> Self {
        match err.kind {
            crate::error::DescribePlacementErrorKind::InternalFailureException(inner) => Error::InternalFailureException(inner),
            crate::error::DescribePlacementErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::error::DescribePlacementErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DescribePlacementErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeProjectError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeProjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeProjectError> for Error {
    fn from(err: crate::error::DescribeProjectError) -> Self {
        match err.kind {
            crate::error::DescribeProjectErrorKind::InternalFailureException(inner) => Error::InternalFailureException(inner),
            crate::error::DescribeProjectErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::error::DescribeProjectErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DescribeProjectErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DisassociateDeviceFromPlacementError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DisassociateDeviceFromPlacementError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DisassociateDeviceFromPlacementError> for Error {
    fn from(err: crate::error::DisassociateDeviceFromPlacementError) -> Self {
        match err.kind {
            crate::error::DisassociateDeviceFromPlacementErrorKind::InternalFailureException(inner) => Error::InternalFailureException(inner),
            crate::error::DisassociateDeviceFromPlacementErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::error::DisassociateDeviceFromPlacementErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DisassociateDeviceFromPlacementErrorKind::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
            crate::error::DisassociateDeviceFromPlacementErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetDevicesInPlacementError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetDevicesInPlacementError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetDevicesInPlacementError> for Error {
    fn from(err: crate::error::GetDevicesInPlacementError) -> Self {
        match err.kind {
            crate::error::GetDevicesInPlacementErrorKind::InternalFailureException(inner) => Error::InternalFailureException(inner),
            crate::error::GetDevicesInPlacementErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::error::GetDevicesInPlacementErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetDevicesInPlacementErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListPlacementsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListPlacementsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListPlacementsError> for Error {
    fn from(err: crate::error::ListPlacementsError) -> Self {
        match err.kind {
            crate::error::ListPlacementsErrorKind::InternalFailureException(inner) => Error::InternalFailureException(inner),
            crate::error::ListPlacementsErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::error::ListPlacementsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::ListPlacementsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListProjectsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListProjectsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListProjectsError> for Error {
    fn from(err: crate::error::ListProjectsError) -> Self {
        match err.kind {
            crate::error::ListProjectsErrorKind::InternalFailureException(inner) => Error::InternalFailureException(inner),
            crate::error::ListProjectsErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::error::ListProjectsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
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
            crate::error::ListTagsForResourceErrorKind::InternalFailureException(inner) => Error::InternalFailureException(inner),
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
            crate::error::TagResourceErrorKind::InternalFailureException(inner) => Error::InternalFailureException(inner),
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
            crate::error::UntagResourceErrorKind::InternalFailureException(inner) => Error::InternalFailureException(inner),
            crate::error::UntagResourceErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdatePlacementError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdatePlacementError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdatePlacementError> for Error {
    fn from(err: crate::error::UpdatePlacementError) -> Self {
        match err.kind {
            crate::error::UpdatePlacementErrorKind::InternalFailureException(inner) => Error::InternalFailureException(inner),
            crate::error::UpdatePlacementErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::error::UpdatePlacementErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UpdatePlacementErrorKind::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
            crate::error::UpdatePlacementErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateProjectError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateProjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateProjectError> for Error {
    fn from(err: crate::error::UpdateProjectError) -> Self {
        match err.kind {
            crate::error::UpdateProjectErrorKind::InternalFailureException(inner) => Error::InternalFailureException(inner),
            crate::error::UpdateProjectErrorKind::InvalidRequestException(inner) => Error::InvalidRequestException(inner),
            crate::error::UpdateProjectErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UpdateProjectErrorKind::TooManyRequestsException(inner) => Error::TooManyRequestsException(inner),
            crate::error::UpdateProjectErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl std::error::Error for Error {}


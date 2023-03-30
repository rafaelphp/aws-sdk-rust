// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request. </p>
    ConflictException(crate::error::ConflictException),
    /// <p>The server encountered an internal error and is unable to complete the request.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>The resource specified in the request was not found.</p>
    NotFoundException(crate::error::NotFoundException),
    /// <p> The resource specified in the request was not found. </p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The request was denied due to request throttling.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p>The input fails to satisfy the specified constraints.</p>
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
            Error::NotFoundException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AssociateRepositoryError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::AssociateRepositoryError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::AssociateRepositoryError> for Error {
    fn from(err: crate::error::AssociateRepositoryError) -> Self {
        match err.kind {
            crate::error::AssociateRepositoryErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::AssociateRepositoryErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::AssociateRepositoryErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::AssociateRepositoryErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::AssociateRepositoryErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::AssociateRepositoryErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateCodeReviewError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateCodeReviewError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateCodeReviewError> for Error {
    fn from(err: crate::error::CreateCodeReviewError) -> Self {
        match err.kind {
            crate::error::CreateCodeReviewErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::CreateCodeReviewErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::CreateCodeReviewErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::CreateCodeReviewErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::CreateCodeReviewErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::CreateCodeReviewErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::CreateCodeReviewErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeCodeReviewError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeCodeReviewError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeCodeReviewError> for Error {
    fn from(err: crate::error::DescribeCodeReviewError) -> Self {
        match err.kind {
            crate::error::DescribeCodeReviewErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DescribeCodeReviewErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::DescribeCodeReviewErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DescribeCodeReviewErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DescribeCodeReviewErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::DescribeCodeReviewErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeRecommendationFeedbackError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeRecommendationFeedbackError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeRecommendationFeedbackError> for Error {
    fn from(err: crate::error::DescribeRecommendationFeedbackError) -> Self {
        match err.kind {
            crate::error::DescribeRecommendationFeedbackErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DescribeRecommendationFeedbackErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::DescribeRecommendationFeedbackErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DescribeRecommendationFeedbackErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DescribeRecommendationFeedbackErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::DescribeRecommendationFeedbackErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeRepositoryAssociationError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeRepositoryAssociationError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeRepositoryAssociationError> for Error {
    fn from(err: crate::error::DescribeRepositoryAssociationError) -> Self {
        match err.kind {
            crate::error::DescribeRepositoryAssociationErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DescribeRepositoryAssociationErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::DescribeRepositoryAssociationErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::DescribeRepositoryAssociationErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DescribeRepositoryAssociationErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::DescribeRepositoryAssociationErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DisassociateRepositoryError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DisassociateRepositoryError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DisassociateRepositoryError> for Error {
    fn from(err: crate::error::DisassociateRepositoryError) -> Self {
        match err.kind {
            crate::error::DisassociateRepositoryErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DisassociateRepositoryErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::DisassociateRepositoryErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::DisassociateRepositoryErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::DisassociateRepositoryErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DisassociateRepositoryErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::DisassociateRepositoryErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListCodeReviewsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListCodeReviewsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListCodeReviewsError> for Error {
    fn from(err: crate::error::ListCodeReviewsError) -> Self {
        match err.kind {
            crate::error::ListCodeReviewsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ListCodeReviewsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::ListCodeReviewsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::ListCodeReviewsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListCodeReviewsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListRecommendationFeedbackError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListRecommendationFeedbackError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListRecommendationFeedbackError> for Error {
    fn from(err: crate::error::ListRecommendationFeedbackError) -> Self {
        match err.kind {
            crate::error::ListRecommendationFeedbackErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ListRecommendationFeedbackErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::ListRecommendationFeedbackErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::ListRecommendationFeedbackErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::ListRecommendationFeedbackErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListRecommendationFeedbackErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListRecommendationsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListRecommendationsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListRecommendationsError> for Error {
    fn from(err: crate::error::ListRecommendationsError) -> Self {
        match err.kind {
            crate::error::ListRecommendationsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ListRecommendationsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::ListRecommendationsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::ListRecommendationsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::ListRecommendationsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListRecommendationsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListRepositoryAssociationsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListRepositoryAssociationsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListRepositoryAssociationsError> for Error {
    fn from(err: crate::error::ListRepositoryAssociationsError) -> Self {
        match err.kind {
            crate::error::ListRepositoryAssociationsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::ListRepositoryAssociationsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::ListRepositoryAssociationsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListRepositoryAssociationsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
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
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutRecommendationFeedbackError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PutRecommendationFeedbackError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::PutRecommendationFeedbackError> for Error {
    fn from(err: crate::error::PutRecommendationFeedbackError) -> Self {
        match err.kind {
            crate::error::PutRecommendationFeedbackErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::PutRecommendationFeedbackErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::PutRecommendationFeedbackErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::PutRecommendationFeedbackErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::PutRecommendationFeedbackErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::PutRecommendationFeedbackErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
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
            crate::error::UntagResourceErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UntagResourceErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl std::error::Error for Error {}


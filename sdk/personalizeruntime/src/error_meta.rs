// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>Provide a valid value for the field or parameter.</p>
    InvalidInputException(crate::error::InvalidInputException),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
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
            Error::InvalidInputException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetPersonalizedRankingError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetPersonalizedRankingError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetPersonalizedRankingError> for Error {
    fn from(err: crate::error::GetPersonalizedRankingError) -> Self {
        match err.kind {
            crate::error::GetPersonalizedRankingErrorKind::InvalidInputException(inner) => Error::InvalidInputException(inner),
            crate::error::GetPersonalizedRankingErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetPersonalizedRankingErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetRecommendationsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetRecommendationsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetRecommendationsError> for Error {
    fn from(err: crate::error::GetRecommendationsError) -> Self {
        match err.kind {
            crate::error::GetRecommendationsErrorKind::InvalidInputException(inner) => Error::InvalidInputException(inner),
            crate::error::GetRecommendationsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetRecommendationsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl std::error::Error for Error {}


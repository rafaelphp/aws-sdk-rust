// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>You are not authorized to use this operation with the given parameters.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>You've exceeded the notification or subscriber limit.</p>
    CreationLimitExceededException(crate::error::CreationLimitExceededException),
    /// <p>The budget name already exists. Budget names must be unique within an account.</p>
    DuplicateRecordException(crate::error::DuplicateRecordException),
    /// <p>The pagination token expired.</p>
    ExpiredNextTokenException(crate::error::ExpiredNextTokenException),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalErrorException(crate::error::InternalErrorException),
    /// <p>The pagination token is invalid.</p>
    InvalidNextTokenException(crate::error::InvalidNextTokenException),
    /// <p>An error on the client occurred. Typically, the cause is an invalid input value.</p>
    InvalidParameterException(crate::error::InvalidParameterException),
    /// <p>We can’t locate the resource that you specified.</p>
    NotFoundException(crate::error::NotFoundException),
    /// <p> The request was received and recognized by the server, but the server rejected that particular method for the requested resource. </p>
    ResourceLockedException(crate::error::ResourceLockedException),
    /// <p> The number of API requests has exceeded the maximum allowed API request throttling limit for the account. </p>
    ThrottlingException(crate::error::ThrottlingException),
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
            Error::CreationLimitExceededException(inner) => inner.fmt(f),
            Error::DuplicateRecordException(inner) => inner.fmt(f),
            Error::ExpiredNextTokenException(inner) => inner.fmt(f),
            Error::InternalErrorException(inner) => inner.fmt(f),
            Error::InvalidNextTokenException(inner) => inner.fmt(f),
            Error::InvalidParameterException(inner) => inner.fmt(f),
            Error::NotFoundException(inner) => inner.fmt(f),
            Error::ResourceLockedException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateBudgetError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateBudgetError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateBudgetError> for Error {
    fn from(err: crate::error::CreateBudgetError) -> Self {
        match err.kind {
            crate::error::CreateBudgetErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::CreateBudgetErrorKind::CreationLimitExceededException(inner) => Error::CreationLimitExceededException(inner),
            crate::error::CreateBudgetErrorKind::DuplicateRecordException(inner) => Error::DuplicateRecordException(inner),
            crate::error::CreateBudgetErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::CreateBudgetErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::CreateBudgetErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::CreateBudgetErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateBudgetActionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateBudgetActionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateBudgetActionError> for Error {
    fn from(err: crate::error::CreateBudgetActionError) -> Self {
        match err.kind {
            crate::error::CreateBudgetActionErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::CreateBudgetActionErrorKind::CreationLimitExceededException(inner) => Error::CreationLimitExceededException(inner),
            crate::error::CreateBudgetActionErrorKind::DuplicateRecordException(inner) => Error::DuplicateRecordException(inner),
            crate::error::CreateBudgetActionErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::CreateBudgetActionErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::CreateBudgetActionErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::CreateBudgetActionErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::CreateBudgetActionErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateNotificationError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateNotificationError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateNotificationError> for Error {
    fn from(err: crate::error::CreateNotificationError) -> Self {
        match err.kind {
            crate::error::CreateNotificationErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::CreateNotificationErrorKind::CreationLimitExceededException(inner) => Error::CreationLimitExceededException(inner),
            crate::error::CreateNotificationErrorKind::DuplicateRecordException(inner) => Error::DuplicateRecordException(inner),
            crate::error::CreateNotificationErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::CreateNotificationErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::CreateNotificationErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::CreateNotificationErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::CreateNotificationErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateSubscriberError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateSubscriberError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateSubscriberError> for Error {
    fn from(err: crate::error::CreateSubscriberError) -> Self {
        match err.kind {
            crate::error::CreateSubscriberErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::CreateSubscriberErrorKind::CreationLimitExceededException(inner) => Error::CreationLimitExceededException(inner),
            crate::error::CreateSubscriberErrorKind::DuplicateRecordException(inner) => Error::DuplicateRecordException(inner),
            crate::error::CreateSubscriberErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::CreateSubscriberErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::CreateSubscriberErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::CreateSubscriberErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::CreateSubscriberErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteBudgetError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteBudgetError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteBudgetError> for Error {
    fn from(err: crate::error::DeleteBudgetError) -> Self {
        match err.kind {
            crate::error::DeleteBudgetErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DeleteBudgetErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::DeleteBudgetErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::DeleteBudgetErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::DeleteBudgetErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DeleteBudgetErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteBudgetActionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteBudgetActionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteBudgetActionError> for Error {
    fn from(err: crate::error::DeleteBudgetActionError) -> Self {
        match err.kind {
            crate::error::DeleteBudgetActionErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DeleteBudgetActionErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::DeleteBudgetActionErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::DeleteBudgetActionErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::DeleteBudgetActionErrorKind::ResourceLockedException(inner) => Error::ResourceLockedException(inner),
            crate::error::DeleteBudgetActionErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DeleteBudgetActionErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteNotificationError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteNotificationError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteNotificationError> for Error {
    fn from(err: crate::error::DeleteNotificationError) -> Self {
        match err.kind {
            crate::error::DeleteNotificationErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DeleteNotificationErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::DeleteNotificationErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::DeleteNotificationErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::DeleteNotificationErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DeleteNotificationErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteSubscriberError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteSubscriberError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteSubscriberError> for Error {
    fn from(err: crate::error::DeleteSubscriberError) -> Self {
        match err.kind {
            crate::error::DeleteSubscriberErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DeleteSubscriberErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::DeleteSubscriberErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::DeleteSubscriberErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::DeleteSubscriberErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DeleteSubscriberErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeBudgetError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeBudgetError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeBudgetError> for Error {
    fn from(err: crate::error::DescribeBudgetError) -> Self {
        match err.kind {
            crate::error::DescribeBudgetErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DescribeBudgetErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::DescribeBudgetErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::DescribeBudgetErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::DescribeBudgetErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DescribeBudgetErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeBudgetActionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeBudgetActionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeBudgetActionError> for Error {
    fn from(err: crate::error::DescribeBudgetActionError) -> Self {
        match err.kind {
            crate::error::DescribeBudgetActionErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DescribeBudgetActionErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::DescribeBudgetActionErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::DescribeBudgetActionErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::DescribeBudgetActionErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DescribeBudgetActionErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeBudgetActionHistoriesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeBudgetActionHistoriesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeBudgetActionHistoriesError> for Error {
    fn from(err: crate::error::DescribeBudgetActionHistoriesError) -> Self {
        match err.kind {
            crate::error::DescribeBudgetActionHistoriesErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DescribeBudgetActionHistoriesErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::DescribeBudgetActionHistoriesErrorKind::InvalidNextTokenException(inner) => Error::InvalidNextTokenException(inner),
            crate::error::DescribeBudgetActionHistoriesErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::DescribeBudgetActionHistoriesErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::DescribeBudgetActionHistoriesErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DescribeBudgetActionHistoriesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeBudgetActionsForAccountError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeBudgetActionsForAccountError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeBudgetActionsForAccountError> for Error {
    fn from(err: crate::error::DescribeBudgetActionsForAccountError) -> Self {
        match err.kind {
            crate::error::DescribeBudgetActionsForAccountErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DescribeBudgetActionsForAccountErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::DescribeBudgetActionsForAccountErrorKind::InvalidNextTokenException(inner) => Error::InvalidNextTokenException(inner),
            crate::error::DescribeBudgetActionsForAccountErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::DescribeBudgetActionsForAccountErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DescribeBudgetActionsForAccountErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeBudgetActionsForBudgetError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeBudgetActionsForBudgetError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeBudgetActionsForBudgetError> for Error {
    fn from(err: crate::error::DescribeBudgetActionsForBudgetError) -> Self {
        match err.kind {
            crate::error::DescribeBudgetActionsForBudgetErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DescribeBudgetActionsForBudgetErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::DescribeBudgetActionsForBudgetErrorKind::InvalidNextTokenException(inner) => Error::InvalidNextTokenException(inner),
            crate::error::DescribeBudgetActionsForBudgetErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::DescribeBudgetActionsForBudgetErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::DescribeBudgetActionsForBudgetErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DescribeBudgetActionsForBudgetErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeBudgetNotificationsForAccountError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeBudgetNotificationsForAccountError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeBudgetNotificationsForAccountError> for Error {
    fn from(err: crate::error::DescribeBudgetNotificationsForAccountError) -> Self {
        match err.kind {
            crate::error::DescribeBudgetNotificationsForAccountErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DescribeBudgetNotificationsForAccountErrorKind::ExpiredNextTokenException(inner) => Error::ExpiredNextTokenException(inner),
            crate::error::DescribeBudgetNotificationsForAccountErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::DescribeBudgetNotificationsForAccountErrorKind::InvalidNextTokenException(inner) => Error::InvalidNextTokenException(inner),
            crate::error::DescribeBudgetNotificationsForAccountErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::DescribeBudgetNotificationsForAccountErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::DescribeBudgetNotificationsForAccountErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DescribeBudgetNotificationsForAccountErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeBudgetPerformanceHistoryError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeBudgetPerformanceHistoryError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeBudgetPerformanceHistoryError> for Error {
    fn from(err: crate::error::DescribeBudgetPerformanceHistoryError) -> Self {
        match err.kind {
            crate::error::DescribeBudgetPerformanceHistoryErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DescribeBudgetPerformanceHistoryErrorKind::ExpiredNextTokenException(inner) => Error::ExpiredNextTokenException(inner),
            crate::error::DescribeBudgetPerformanceHistoryErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::DescribeBudgetPerformanceHistoryErrorKind::InvalidNextTokenException(inner) => Error::InvalidNextTokenException(inner),
            crate::error::DescribeBudgetPerformanceHistoryErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::DescribeBudgetPerformanceHistoryErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::DescribeBudgetPerformanceHistoryErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DescribeBudgetPerformanceHistoryErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeBudgetsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeBudgetsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeBudgetsError> for Error {
    fn from(err: crate::error::DescribeBudgetsError) -> Self {
        match err.kind {
            crate::error::DescribeBudgetsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DescribeBudgetsErrorKind::ExpiredNextTokenException(inner) => Error::ExpiredNextTokenException(inner),
            crate::error::DescribeBudgetsErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::DescribeBudgetsErrorKind::InvalidNextTokenException(inner) => Error::InvalidNextTokenException(inner),
            crate::error::DescribeBudgetsErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::DescribeBudgetsErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::DescribeBudgetsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DescribeBudgetsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeNotificationsForBudgetError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeNotificationsForBudgetError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeNotificationsForBudgetError> for Error {
    fn from(err: crate::error::DescribeNotificationsForBudgetError) -> Self {
        match err.kind {
            crate::error::DescribeNotificationsForBudgetErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DescribeNotificationsForBudgetErrorKind::ExpiredNextTokenException(inner) => Error::ExpiredNextTokenException(inner),
            crate::error::DescribeNotificationsForBudgetErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::DescribeNotificationsForBudgetErrorKind::InvalidNextTokenException(inner) => Error::InvalidNextTokenException(inner),
            crate::error::DescribeNotificationsForBudgetErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::DescribeNotificationsForBudgetErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::DescribeNotificationsForBudgetErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DescribeNotificationsForBudgetErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeSubscribersForNotificationError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeSubscribersForNotificationError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeSubscribersForNotificationError> for Error {
    fn from(err: crate::error::DescribeSubscribersForNotificationError) -> Self {
        match err.kind {
            crate::error::DescribeSubscribersForNotificationErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DescribeSubscribersForNotificationErrorKind::ExpiredNextTokenException(inner) => Error::ExpiredNextTokenException(inner),
            crate::error::DescribeSubscribersForNotificationErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::DescribeSubscribersForNotificationErrorKind::InvalidNextTokenException(inner) => Error::InvalidNextTokenException(inner),
            crate::error::DescribeSubscribersForNotificationErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::DescribeSubscribersForNotificationErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::DescribeSubscribersForNotificationErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DescribeSubscribersForNotificationErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ExecuteBudgetActionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ExecuteBudgetActionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ExecuteBudgetActionError> for Error {
    fn from(err: crate::error::ExecuteBudgetActionError) -> Self {
        match err.kind {
            crate::error::ExecuteBudgetActionErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ExecuteBudgetActionErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::ExecuteBudgetActionErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::ExecuteBudgetActionErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::ExecuteBudgetActionErrorKind::ResourceLockedException(inner) => Error::ResourceLockedException(inner),
            crate::error::ExecuteBudgetActionErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::ExecuteBudgetActionErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateBudgetError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateBudgetError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateBudgetError> for Error {
    fn from(err: crate::error::UpdateBudgetError) -> Self {
        match err.kind {
            crate::error::UpdateBudgetErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::UpdateBudgetErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::UpdateBudgetErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::UpdateBudgetErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::UpdateBudgetErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::UpdateBudgetErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateBudgetActionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateBudgetActionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateBudgetActionError> for Error {
    fn from(err: crate::error::UpdateBudgetActionError) -> Self {
        match err.kind {
            crate::error::UpdateBudgetActionErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::UpdateBudgetActionErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::UpdateBudgetActionErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::UpdateBudgetActionErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::UpdateBudgetActionErrorKind::ResourceLockedException(inner) => Error::ResourceLockedException(inner),
            crate::error::UpdateBudgetActionErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::UpdateBudgetActionErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateNotificationError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateNotificationError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateNotificationError> for Error {
    fn from(err: crate::error::UpdateNotificationError) -> Self {
        match err.kind {
            crate::error::UpdateNotificationErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::UpdateNotificationErrorKind::DuplicateRecordException(inner) => Error::DuplicateRecordException(inner),
            crate::error::UpdateNotificationErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::UpdateNotificationErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::UpdateNotificationErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::UpdateNotificationErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::UpdateNotificationErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateSubscriberError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateSubscriberError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateSubscriberError> for Error {
    fn from(err: crate::error::UpdateSubscriberError) -> Self {
        match err.kind {
            crate::error::UpdateSubscriberErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::UpdateSubscriberErrorKind::DuplicateRecordException(inner) => Error::DuplicateRecordException(inner),
            crate::error::UpdateSubscriberErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::UpdateSubscriberErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::UpdateSubscriberErrorKind::NotFoundException(inner) => Error::NotFoundException(inner),
            crate::error::UpdateSubscriberErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::UpdateSubscriberErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl std::error::Error for Error {}


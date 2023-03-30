// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>AWS CodeStar Notifications can't create the notification rule because you do not have sufficient permissions.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>AWS CodeStar Notifications can't complete the request because the resource is being modified by another process. Wait a few minutes and try again.</p>
    ConcurrentModificationException(crate::error::ConcurrentModificationException),
    /// <p>Some or all of the configuration is incomplete, missing, or not valid.</p>
    ConfigurationException(crate::error::ConfigurationException),
    /// <p>The value for the enumeration token used in the request to return the next batch of the results is not valid. </p>
    InvalidNextTokenException(crate::error::InvalidNextTokenException),
    /// <p>One of the AWS CodeStar Notifications limits has been exceeded. Limits apply to accounts, notification rules, notifications, resources, and targets. For more information, see Limits.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>A resource with the same name or ID already exists. Notification rule names must be unique in your Amazon Web Services account.</p>
    ResourceAlreadyExistsException(crate::error::ResourceAlreadyExistsException),
    /// <p>AWS CodeStar Notifications can't find a resource that matches the provided ARN. </p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>One or more parameter values are not valid.</p>
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
            Error::ConcurrentModificationException(inner) => inner.fmt(f),
            Error::ConfigurationException(inner) => inner.fmt(f),
            Error::InvalidNextTokenException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ResourceAlreadyExistsException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateNotificationRuleError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateNotificationRuleError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateNotificationRuleError> for Error {
    fn from(err: crate::error::CreateNotificationRuleError) -> Self {
        match err.kind {
            crate::error::CreateNotificationRuleErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::CreateNotificationRuleErrorKind::ConcurrentModificationException(inner) => Error::ConcurrentModificationException(inner),
            crate::error::CreateNotificationRuleErrorKind::ConfigurationException(inner) => Error::ConfigurationException(inner),
            crate::error::CreateNotificationRuleErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
            crate::error::CreateNotificationRuleErrorKind::ResourceAlreadyExistsException(inner) => Error::ResourceAlreadyExistsException(inner),
            crate::error::CreateNotificationRuleErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::CreateNotificationRuleErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteNotificationRuleError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteNotificationRuleError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteNotificationRuleError> for Error {
    fn from(err: crate::error::DeleteNotificationRuleError) -> Self {
        match err.kind {
            crate::error::DeleteNotificationRuleErrorKind::ConcurrentModificationException(inner) => Error::ConcurrentModificationException(inner),
            crate::error::DeleteNotificationRuleErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
            crate::error::DeleteNotificationRuleErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::DeleteNotificationRuleErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteTargetError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteTargetError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteTargetError> for Error {
    fn from(err: crate::error::DeleteTargetError) -> Self {
        match err.kind {
            crate::error::DeleteTargetErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::DeleteTargetErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeNotificationRuleError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeNotificationRuleError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeNotificationRuleError> for Error {
    fn from(err: crate::error::DescribeNotificationRuleError) -> Self {
        match err.kind {
            crate::error::DescribeNotificationRuleErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DescribeNotificationRuleErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::DescribeNotificationRuleErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListEventTypesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListEventTypesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListEventTypesError> for Error {
    fn from(err: crate::error::ListEventTypesError) -> Self {
        match err.kind {
            crate::error::ListEventTypesErrorKind::InvalidNextTokenException(inner) => Error::InvalidNextTokenException(inner),
            crate::error::ListEventTypesErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListEventTypesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListNotificationRulesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListNotificationRulesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListNotificationRulesError> for Error {
    fn from(err: crate::error::ListNotificationRulesError) -> Self {
        match err.kind {
            crate::error::ListNotificationRulesErrorKind::InvalidNextTokenException(inner) => Error::InvalidNextTokenException(inner),
            crate::error::ListNotificationRulesErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListNotificationRulesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
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
            crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::ListTagsForResourceErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListTargetsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListTargetsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListTargetsError> for Error {
    fn from(err: crate::error::ListTargetsError) -> Self {
        match err.kind {
            crate::error::ListTargetsErrorKind::InvalidNextTokenException(inner) => Error::InvalidNextTokenException(inner),
            crate::error::ListTargetsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListTargetsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SubscribeError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::SubscribeError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::SubscribeError> for Error {
    fn from(err: crate::error::SubscribeError) -> Self {
        match err.kind {
            crate::error::SubscribeErrorKind::ConfigurationException(inner) => Error::ConfigurationException(inner),
            crate::error::SubscribeErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::SubscribeErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::SubscribeErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
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
            crate::error::TagResourceErrorKind::ConcurrentModificationException(inner) => Error::ConcurrentModificationException(inner),
            crate::error::TagResourceErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
            crate::error::TagResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::TagResourceErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::TagResourceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UnsubscribeError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UnsubscribeError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UnsubscribeError> for Error {
    fn from(err: crate::error::UnsubscribeError) -> Self {
        match err.kind {
            crate::error::UnsubscribeErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::UnsubscribeErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
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
            crate::error::UntagResourceErrorKind::ConcurrentModificationException(inner) => Error::ConcurrentModificationException(inner),
            crate::error::UntagResourceErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
            crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UntagResourceErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateNotificationRuleError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateNotificationRuleError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateNotificationRuleError> for Error {
    fn from(err: crate::error::UpdateNotificationRuleError) -> Self {
        match err.kind {
            crate::error::UpdateNotificationRuleErrorKind::ConfigurationException(inner) => Error::ConfigurationException(inner),
            crate::error::UpdateNotificationRuleErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UpdateNotificationRuleErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::UpdateNotificationRuleErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl std::error::Error for Error {}


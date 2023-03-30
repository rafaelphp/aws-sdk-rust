// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>(Discontinued) You do not have required permissions to access the requested resource.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>(Discontinued) Internal server error.</p>
    InternalException(crate::error::InternalException),
    /// <p>(Discontinued) The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInputException(crate::error::InvalidInputException),
    /// <p>(Discontinued) The request was rejected because it attempted to create resources beyond the current Amazon Web Services account quotas. The error code describes the quota exceeded.</p>
    LimitExceededException(crate::error::LimitExceededException),
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
            Error::InternalException(inner) => inner.fmt(f),
            Error::InvalidInputException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AssociateMemberAccountError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::AssociateMemberAccountError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::AssociateMemberAccountError> for Error {
    fn from(err: crate::error::AssociateMemberAccountError) -> Self {
        match err.kind {
            crate::error::AssociateMemberAccountErrorKind::InternalException(inner) => Error::InternalException(inner),
            crate::error::AssociateMemberAccountErrorKind::InvalidInputException(inner) => Error::InvalidInputException(inner),
            crate::error::AssociateMemberAccountErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
            crate::error::AssociateMemberAccountErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AssociateS3ResourcesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::AssociateS3ResourcesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::AssociateS3ResourcesError> for Error {
    fn from(err: crate::error::AssociateS3ResourcesError) -> Self {
        match err.kind {
            crate::error::AssociateS3ResourcesErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::AssociateS3ResourcesErrorKind::InternalException(inner) => Error::InternalException(inner),
            crate::error::AssociateS3ResourcesErrorKind::InvalidInputException(inner) => Error::InvalidInputException(inner),
            crate::error::AssociateS3ResourcesErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
            crate::error::AssociateS3ResourcesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DisassociateMemberAccountError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DisassociateMemberAccountError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DisassociateMemberAccountError> for Error {
    fn from(err: crate::error::DisassociateMemberAccountError) -> Self {
        match err.kind {
            crate::error::DisassociateMemberAccountErrorKind::InternalException(inner) => Error::InternalException(inner),
            crate::error::DisassociateMemberAccountErrorKind::InvalidInputException(inner) => Error::InvalidInputException(inner),
            crate::error::DisassociateMemberAccountErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DisassociateS3ResourcesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DisassociateS3ResourcesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DisassociateS3ResourcesError> for Error {
    fn from(err: crate::error::DisassociateS3ResourcesError) -> Self {
        match err.kind {
            crate::error::DisassociateS3ResourcesErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DisassociateS3ResourcesErrorKind::InternalException(inner) => Error::InternalException(inner),
            crate::error::DisassociateS3ResourcesErrorKind::InvalidInputException(inner) => Error::InvalidInputException(inner),
            crate::error::DisassociateS3ResourcesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListMemberAccountsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListMemberAccountsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListMemberAccountsError> for Error {
    fn from(err: crate::error::ListMemberAccountsError) -> Self {
        match err.kind {
            crate::error::ListMemberAccountsErrorKind::InternalException(inner) => Error::InternalException(inner),
            crate::error::ListMemberAccountsErrorKind::InvalidInputException(inner) => Error::InvalidInputException(inner),
            crate::error::ListMemberAccountsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListS3ResourcesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListS3ResourcesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListS3ResourcesError> for Error {
    fn from(err: crate::error::ListS3ResourcesError) -> Self {
        match err.kind {
            crate::error::ListS3ResourcesErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ListS3ResourcesErrorKind::InternalException(inner) => Error::InternalException(inner),
            crate::error::ListS3ResourcesErrorKind::InvalidInputException(inner) => Error::InvalidInputException(inner),
            crate::error::ListS3ResourcesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateS3ResourcesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateS3ResourcesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateS3ResourcesError> for Error {
    fn from(err: crate::error::UpdateS3ResourcesError) -> Self {
        match err.kind {
            crate::error::UpdateS3ResourcesErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::UpdateS3ResourcesErrorKind::InternalException(inner) => Error::InternalException(inner),
            crate::error::UpdateS3ResourcesErrorKind::InvalidInputException(inner) => Error::InvalidInputException(inner),
            crate::error::UpdateS3ResourcesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl std::error::Error for Error {}


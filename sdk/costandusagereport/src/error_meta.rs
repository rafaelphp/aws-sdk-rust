// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>A report with the specified name already exists in the account. Specify a different report name.</p>
    DuplicateReportNameException(crate::error::DuplicateReportNameException),
    /// <p>An error on the server occurred during the processing of your request. Try again later.</p>
    InternalErrorException(crate::error::InternalErrorException),
    /// <p>This account already has five reports defined. To define a new report, you must delete an existing report.</p>
    ReportLimitReachedException(crate::error::ReportLimitReachedException),
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
            Error::DuplicateReportNameException(inner) => inner.fmt(f),
            Error::InternalErrorException(inner) => inner.fmt(f),
            Error::ReportLimitReachedException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteReportDefinitionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteReportDefinitionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteReportDefinitionError> for Error {
    fn from(err: crate::error::DeleteReportDefinitionError) -> Self {
        match err.kind {
            crate::error::DeleteReportDefinitionErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::DeleteReportDefinitionErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::DeleteReportDefinitionErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeReportDefinitionsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeReportDefinitionsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeReportDefinitionsError> for Error {
    fn from(err: crate::error::DescribeReportDefinitionsError) -> Self {
        match err.kind {
            crate::error::DescribeReportDefinitionsErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::DescribeReportDefinitionsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ModifyReportDefinitionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ModifyReportDefinitionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ModifyReportDefinitionError> for Error {
    fn from(err: crate::error::ModifyReportDefinitionError) -> Self {
        match err.kind {
            crate::error::ModifyReportDefinitionErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::ModifyReportDefinitionErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ModifyReportDefinitionErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutReportDefinitionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PutReportDefinitionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::PutReportDefinitionError> for Error {
    fn from(err: crate::error::PutReportDefinitionError) -> Self {
        match err.kind {
            crate::error::PutReportDefinitionErrorKind::DuplicateReportNameException(inner) => Error::DuplicateReportNameException(inner),
            crate::error::PutReportDefinitionErrorKind::InternalErrorException(inner) => Error::InternalErrorException(inner),
            crate::error::PutReportDefinitionErrorKind::ReportLimitReachedException(inner) => Error::ReportLimitReachedException(inner),
            crate::error::PutReportDefinitionErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::PutReportDefinitionErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl std::error::Error for Error {}


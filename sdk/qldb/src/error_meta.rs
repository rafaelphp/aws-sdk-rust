// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>One or more parameters in the request aren't valid.</p>
    InvalidParameterException(crate::error::InvalidParameterException),
    /// <p>You have reached the limit on the maximum number of resources allowed.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExistsException(crate::error::ResourceAlreadyExistsException),
    /// <p>The specified resource can't be modified at this time.</p>
    ResourceInUseException(crate::error::ResourceInUseException),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The operation failed because a condition wasn't satisfied in advance.</p>
    ResourcePreconditionNotMetException(crate::error::ResourcePreconditionNotMetException),
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
            Error::InvalidParameterException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ResourceAlreadyExistsException(inner) => inner.fmt(f),
            Error::ResourceInUseException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ResourcePreconditionNotMetException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CancelJournalKinesisStreamError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CancelJournalKinesisStreamError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CancelJournalKinesisStreamError> for Error {
    fn from(err: crate::error::CancelJournalKinesisStreamError) -> Self {
        match err.kind {
            crate::error::CancelJournalKinesisStreamErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::CancelJournalKinesisStreamErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::CancelJournalKinesisStreamErrorKind::ResourcePreconditionNotMetException(inner) => Error::ResourcePreconditionNotMetException(inner),
            crate::error::CancelJournalKinesisStreamErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateLedgerError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateLedgerError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateLedgerError> for Error {
    fn from(err: crate::error::CreateLedgerError) -> Self {
        match err.kind {
            crate::error::CreateLedgerErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::CreateLedgerErrorKind::LimitExceededException(inner) => Error::LimitExceededException(inner),
            crate::error::CreateLedgerErrorKind::ResourceAlreadyExistsException(inner) => Error::ResourceAlreadyExistsException(inner),
            crate::error::CreateLedgerErrorKind::ResourceInUseException(inner) => Error::ResourceInUseException(inner),
            crate::error::CreateLedgerErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteLedgerError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteLedgerError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteLedgerError> for Error {
    fn from(err: crate::error::DeleteLedgerError) -> Self {
        match err.kind {
            crate::error::DeleteLedgerErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::DeleteLedgerErrorKind::ResourceInUseException(inner) => Error::ResourceInUseException(inner),
            crate::error::DeleteLedgerErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeleteLedgerErrorKind::ResourcePreconditionNotMetException(inner) => Error::ResourcePreconditionNotMetException(inner),
            crate::error::DeleteLedgerErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeJournalKinesisStreamError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeJournalKinesisStreamError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeJournalKinesisStreamError> for Error {
    fn from(err: crate::error::DescribeJournalKinesisStreamError) -> Self {
        match err.kind {
            crate::error::DescribeJournalKinesisStreamErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::DescribeJournalKinesisStreamErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DescribeJournalKinesisStreamErrorKind::ResourcePreconditionNotMetException(inner) => Error::ResourcePreconditionNotMetException(inner),
            crate::error::DescribeJournalKinesisStreamErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeJournalS3ExportError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeJournalS3ExportError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeJournalS3ExportError> for Error {
    fn from(err: crate::error::DescribeJournalS3ExportError) -> Self {
        match err.kind {
            crate::error::DescribeJournalS3ExportErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DescribeJournalS3ExportErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeLedgerError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeLedgerError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeLedgerError> for Error {
    fn from(err: crate::error::DescribeLedgerError) -> Self {
        match err.kind {
            crate::error::DescribeLedgerErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::DescribeLedgerErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DescribeLedgerErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ExportJournalToS3Error, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ExportJournalToS3Error, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ExportJournalToS3Error> for Error {
    fn from(err: crate::error::ExportJournalToS3Error) -> Self {
        match err.kind {
            crate::error::ExportJournalToS3ErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::ExportJournalToS3ErrorKind::ResourcePreconditionNotMetException(inner) => Error::ResourcePreconditionNotMetException(inner),
            crate::error::ExportJournalToS3ErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetBlockError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetBlockError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetBlockError> for Error {
    fn from(err: crate::error::GetBlockError) -> Self {
        match err.kind {
            crate::error::GetBlockErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::GetBlockErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetBlockErrorKind::ResourcePreconditionNotMetException(inner) => Error::ResourcePreconditionNotMetException(inner),
            crate::error::GetBlockErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetDigestError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetDigestError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetDigestError> for Error {
    fn from(err: crate::error::GetDigestError) -> Self {
        match err.kind {
            crate::error::GetDigestErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::GetDigestErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetDigestErrorKind::ResourcePreconditionNotMetException(inner) => Error::ResourcePreconditionNotMetException(inner),
            crate::error::GetDigestErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetRevisionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetRevisionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetRevisionError> for Error {
    fn from(err: crate::error::GetRevisionError) -> Self {
        match err.kind {
            crate::error::GetRevisionErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::GetRevisionErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetRevisionErrorKind::ResourcePreconditionNotMetException(inner) => Error::ResourcePreconditionNotMetException(inner),
            crate::error::GetRevisionErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListJournalKinesisStreamsForLedgerError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListJournalKinesisStreamsForLedgerError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListJournalKinesisStreamsForLedgerError> for Error {
    fn from(err: crate::error::ListJournalKinesisStreamsForLedgerError) -> Self {
        match err.kind {
            crate::error::ListJournalKinesisStreamsForLedgerErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::ListJournalKinesisStreamsForLedgerErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::ListJournalKinesisStreamsForLedgerErrorKind::ResourcePreconditionNotMetException(inner) => Error::ResourcePreconditionNotMetException(inner),
            crate::error::ListJournalKinesisStreamsForLedgerErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListJournalS3ExportsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListJournalS3ExportsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListJournalS3ExportsError> for Error {
    fn from(err: crate::error::ListJournalS3ExportsError) -> Self {
        match err.kind {
            crate::error::ListJournalS3ExportsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListJournalS3ExportsForLedgerError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListJournalS3ExportsForLedgerError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListJournalS3ExportsForLedgerError> for Error {
    fn from(err: crate::error::ListJournalS3ExportsForLedgerError) -> Self {
        match err.kind {
            crate::error::ListJournalS3ExportsForLedgerErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListLedgersError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListLedgersError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListLedgersError> for Error {
    fn from(err: crate::error::ListLedgersError) -> Self {
        match err.kind {
            crate::error::ListLedgersErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
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
            crate::error::ListTagsForResourceErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::ListTagsForResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::ListTagsForResourceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StreamJournalToKinesisError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::StreamJournalToKinesisError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::StreamJournalToKinesisError> for Error {
    fn from(err: crate::error::StreamJournalToKinesisError) -> Self {
        match err.kind {
            crate::error::StreamJournalToKinesisErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::StreamJournalToKinesisErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::StreamJournalToKinesisErrorKind::ResourcePreconditionNotMetException(inner) => Error::ResourcePreconditionNotMetException(inner),
            crate::error::StreamJournalToKinesisErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
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
            crate::error::TagResourceErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
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
            crate::error::UntagResourceErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::UntagResourceErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UntagResourceErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateLedgerError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateLedgerError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateLedgerError> for Error {
    fn from(err: crate::error::UpdateLedgerError) -> Self {
        match err.kind {
            crate::error::UpdateLedgerErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::UpdateLedgerErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UpdateLedgerErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateLedgerPermissionsModeError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateLedgerPermissionsModeError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateLedgerPermissionsModeError> for Error {
    fn from(err: crate::error::UpdateLedgerPermissionsModeError) -> Self {
        match err.kind {
            crate::error::UpdateLedgerPermissionsModeErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::UpdateLedgerPermissionsModeErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UpdateLedgerPermissionsModeErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl std::error::Error for Error {}


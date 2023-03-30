// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p></p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p></p>
    ChannelNotBroadcasting(crate::error::ChannelNotBroadcasting),
    /// <p></p>
    ConflictException(crate::error::ConflictException),
    /// <p></p>
    InternalServerException(crate::error::InternalServerException),
    /// <p></p>
    PendingVerification(crate::error::PendingVerification),
    /// <p></p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p></p>
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    /// <p></p>
    StreamUnavailable(crate::error::StreamUnavailable),
    /// <p></p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p></p>
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
            Error::ChannelNotBroadcasting(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::PendingVerification(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::StreamUnavailable(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::BatchGetChannelError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::BatchGetChannelError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::BatchGetChannelError> for Error {
    fn from(err: crate::error::BatchGetChannelError) -> Self {
        match err.kind {
            crate::error::BatchGetChannelErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::BatchGetStreamKeyError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::BatchGetStreamKeyError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::BatchGetStreamKeyError> for Error {
    fn from(err: crate::error::BatchGetStreamKeyError) -> Self {
        match err.kind {
            crate::error::BatchGetStreamKeyErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateChannelError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateChannelError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateChannelError> for Error {
    fn from(err: crate::error::CreateChannelError) -> Self {
        match err.kind {
            crate::error::CreateChannelErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::CreateChannelErrorKind::PendingVerification(inner) => Error::PendingVerification(inner),
            crate::error::CreateChannelErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::CreateChannelErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::error::CreateChannelErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::CreateChannelErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateRecordingConfigurationError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateRecordingConfigurationError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateRecordingConfigurationError> for Error {
    fn from(err: crate::error::CreateRecordingConfigurationError) -> Self {
        match err.kind {
            crate::error::CreateRecordingConfigurationErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::CreateRecordingConfigurationErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::CreateRecordingConfigurationErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::CreateRecordingConfigurationErrorKind::PendingVerification(inner) => Error::PendingVerification(inner),
            crate::error::CreateRecordingConfigurationErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::error::CreateRecordingConfigurationErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::CreateRecordingConfigurationErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateStreamKeyError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateStreamKeyError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateStreamKeyError> for Error {
    fn from(err: crate::error::CreateStreamKeyError) -> Self {
        match err.kind {
            crate::error::CreateStreamKeyErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::CreateStreamKeyErrorKind::PendingVerification(inner) => Error::PendingVerification(inner),
            crate::error::CreateStreamKeyErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::CreateStreamKeyErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::error::CreateStreamKeyErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::CreateStreamKeyErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteChannelError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteChannelError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteChannelError> for Error {
    fn from(err: crate::error::DeleteChannelError) -> Self {
        match err.kind {
            crate::error::DeleteChannelErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DeleteChannelErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::DeleteChannelErrorKind::PendingVerification(inner) => Error::PendingVerification(inner),
            crate::error::DeleteChannelErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeleteChannelErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::DeleteChannelErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeletePlaybackKeyPairError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeletePlaybackKeyPairError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeletePlaybackKeyPairError> for Error {
    fn from(err: crate::error::DeletePlaybackKeyPairError) -> Self {
        match err.kind {
            crate::error::DeletePlaybackKeyPairErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DeletePlaybackKeyPairErrorKind::PendingVerification(inner) => Error::PendingVerification(inner),
            crate::error::DeletePlaybackKeyPairErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeletePlaybackKeyPairErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::DeletePlaybackKeyPairErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteRecordingConfigurationError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteRecordingConfigurationError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteRecordingConfigurationError> for Error {
    fn from(err: crate::error::DeleteRecordingConfigurationError) -> Self {
        match err.kind {
            crate::error::DeleteRecordingConfigurationErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DeleteRecordingConfigurationErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::DeleteRecordingConfigurationErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::DeleteRecordingConfigurationErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeleteRecordingConfigurationErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::DeleteRecordingConfigurationErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteStreamKeyError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteStreamKeyError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteStreamKeyError> for Error {
    fn from(err: crate::error::DeleteStreamKeyError) -> Self {
        match err.kind {
            crate::error::DeleteStreamKeyErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DeleteStreamKeyErrorKind::PendingVerification(inner) => Error::PendingVerification(inner),
            crate::error::DeleteStreamKeyErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeleteStreamKeyErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::DeleteStreamKeyErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetChannelError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetChannelError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetChannelError> for Error {
    fn from(err: crate::error::GetChannelError) -> Self {
        match err.kind {
            crate::error::GetChannelErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::GetChannelErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetChannelErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::GetChannelErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetPlaybackKeyPairError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetPlaybackKeyPairError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetPlaybackKeyPairError> for Error {
    fn from(err: crate::error::GetPlaybackKeyPairError) -> Self {
        match err.kind {
            crate::error::GetPlaybackKeyPairErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::GetPlaybackKeyPairErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetPlaybackKeyPairErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::GetPlaybackKeyPairErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetRecordingConfigurationError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetRecordingConfigurationError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetRecordingConfigurationError> for Error {
    fn from(err: crate::error::GetRecordingConfigurationError) -> Self {
        match err.kind {
            crate::error::GetRecordingConfigurationErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::GetRecordingConfigurationErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::GetRecordingConfigurationErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetRecordingConfigurationErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::GetRecordingConfigurationErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetStreamError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetStreamError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetStreamError> for Error {
    fn from(err: crate::error::GetStreamError) -> Self {
        match err.kind {
            crate::error::GetStreamErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::GetStreamErrorKind::ChannelNotBroadcasting(inner) => Error::ChannelNotBroadcasting(inner),
            crate::error::GetStreamErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetStreamErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::GetStreamErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetStreamKeyError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetStreamKeyError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetStreamKeyError> for Error {
    fn from(err: crate::error::GetStreamKeyError) -> Self {
        match err.kind {
            crate::error::GetStreamKeyErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::GetStreamKeyErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetStreamKeyErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::GetStreamKeyErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetStreamSessionError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetStreamSessionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetStreamSessionError> for Error {
    fn from(err: crate::error::GetStreamSessionError) -> Self {
        match err.kind {
            crate::error::GetStreamSessionErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::GetStreamSessionErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetStreamSessionErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::GetStreamSessionErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ImportPlaybackKeyPairError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ImportPlaybackKeyPairError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ImportPlaybackKeyPairError> for Error {
    fn from(err: crate::error::ImportPlaybackKeyPairError) -> Self {
        match err.kind {
            crate::error::ImportPlaybackKeyPairErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ImportPlaybackKeyPairErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::ImportPlaybackKeyPairErrorKind::PendingVerification(inner) => Error::PendingVerification(inner),
            crate::error::ImportPlaybackKeyPairErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::error::ImportPlaybackKeyPairErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ImportPlaybackKeyPairErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListChannelsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListChannelsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListChannelsError> for Error {
    fn from(err: crate::error::ListChannelsError) -> Self {
        match err.kind {
            crate::error::ListChannelsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ListChannelsErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::ListChannelsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListChannelsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListPlaybackKeyPairsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListPlaybackKeyPairsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListPlaybackKeyPairsError> for Error {
    fn from(err: crate::error::ListPlaybackKeyPairsError) -> Self {
        match err.kind {
            crate::error::ListPlaybackKeyPairsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ListPlaybackKeyPairsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListPlaybackKeyPairsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListRecordingConfigurationsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListRecordingConfigurationsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListRecordingConfigurationsError> for Error {
    fn from(err: crate::error::ListRecordingConfigurationsError) -> Self {
        match err.kind {
            crate::error::ListRecordingConfigurationsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ListRecordingConfigurationsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::ListRecordingConfigurationsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListRecordingConfigurationsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListStreamKeysError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListStreamKeysError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListStreamKeysError> for Error {
    fn from(err: crate::error::ListStreamKeysError) -> Self {
        match err.kind {
            crate::error::ListStreamKeysErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ListStreamKeysErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::ListStreamKeysErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListStreamKeysErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListStreamsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListStreamsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListStreamsError> for Error {
    fn from(err: crate::error::ListStreamsError) -> Self {
        match err.kind {
            crate::error::ListStreamsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ListStreamsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListStreamsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListStreamSessionsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListStreamSessionsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListStreamSessionsError> for Error {
    fn from(err: crate::error::ListStreamSessionsError) -> Self {
        match err.kind {
            crate::error::ListStreamSessionsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ListStreamSessionsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::ListStreamSessionsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::ListStreamSessionsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
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
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutMetadataError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PutMetadataError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::PutMetadataError> for Error {
    fn from(err: crate::error::PutMetadataError) -> Self {
        match err.kind {
            crate::error::PutMetadataErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::PutMetadataErrorKind::ChannelNotBroadcasting(inner) => Error::ChannelNotBroadcasting(inner),
            crate::error::PutMetadataErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::PutMetadataErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::PutMetadataErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::PutMetadataErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StopStreamError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::StopStreamError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::StopStreamError> for Error {
    fn from(err: crate::error::StopStreamError) -> Self {
        match err.kind {
            crate::error::StopStreamErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::StopStreamErrorKind::ChannelNotBroadcasting(inner) => Error::ChannelNotBroadcasting(inner),
            crate::error::StopStreamErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::StopStreamErrorKind::StreamUnavailable(inner) => Error::StreamUnavailable(inner),
            crate::error::StopStreamErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::StopStreamErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
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
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateChannelError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateChannelError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateChannelError> for Error {
    fn from(err: crate::error::UpdateChannelError) -> Self {
        match err.kind {
            crate::error::UpdateChannelErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::UpdateChannelErrorKind::ConflictException(inner) => Error::ConflictException(inner),
            crate::error::UpdateChannelErrorKind::PendingVerification(inner) => Error::PendingVerification(inner),
            crate::error::UpdateChannelErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::UpdateChannelErrorKind::ValidationException(inner) => Error::ValidationException(inner),
            crate::error::UpdateChannelErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl std::error::Error for Error {}


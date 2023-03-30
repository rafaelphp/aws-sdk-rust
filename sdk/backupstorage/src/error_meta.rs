// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    #[allow(missing_docs)] // documentation missing in model
    AccessDeniedException(crate::error::AccessDeniedException),
    /// Non-retryable exception. Attempted to create already existing object or chunk. This message contains a checksum of already presented data.
    DataAlreadyExistsException(crate::error::DataAlreadyExistsException),
    /// Non-retryable exception, indicates client error (wrong argument passed to API). See exception message for details.
    IllegalArgumentException(crate::error::IllegalArgumentException),
    /// Non-retryable exception. Indicates the KMS key usage is incorrect. See exception message for details.
    KmsInvalidKeyUsageException(crate::error::KmsInvalidKeyUsageException),
    /// Retryalble exception. Indicated issues while reading an input stream due to the networking issues or connection drop on the client side.
    NotReadableInputStreamException(crate::error::NotReadableInputStreamException),
    /// Non-retryable exception. Attempted to make an operation on non-existing or expired resource.
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// Retryable exception. In general indicates internal failure that can be fixed by retry.
    RetryableException(crate::error::RetryableException),
    /// Deprecated. To be removed from the model.
    ServiceInternalException(crate::error::ServiceInternalException),
    /// Retryable exception, indicates internal server error.
    ServiceUnavailableException(crate::error::ServiceUnavailableException),
    /// Increased rate over throttling limits. Can be retried with exponential backoff.
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
            Error::DataAlreadyExistsException(inner) => inner.fmt(f),
            Error::IllegalArgumentException(inner) => inner.fmt(f),
            Error::KmsInvalidKeyUsageException(inner) => inner.fmt(f),
            Error::NotReadableInputStreamException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::RetryableException(inner) => inner.fmt(f),
            Error::ServiceInternalException(inner) => inner.fmt(f),
            Error::ServiceUnavailableException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteObjectError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteObjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteObjectError> for Error {
    fn from(err: crate::error::DeleteObjectError) -> Self {
        match err.kind {
            crate::error::DeleteObjectErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::DeleteObjectErrorKind::IllegalArgumentException(inner) => Error::IllegalArgumentException(inner),
            crate::error::DeleteObjectErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeleteObjectErrorKind::RetryableException(inner) => Error::RetryableException(inner),
            crate::error::DeleteObjectErrorKind::ServiceInternalException(inner) => Error::ServiceInternalException(inner),
            crate::error::DeleteObjectErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
            crate::error::DeleteObjectErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::DeleteObjectErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetChunkError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetChunkError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetChunkError> for Error {
    fn from(err: crate::error::GetChunkError) -> Self {
        match err.kind {
            crate::error::GetChunkErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::GetChunkErrorKind::IllegalArgumentException(inner) => Error::IllegalArgumentException(inner),
            crate::error::GetChunkErrorKind::KmsInvalidKeyUsageException(inner) => Error::KmsInvalidKeyUsageException(inner),
            crate::error::GetChunkErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetChunkErrorKind::RetryableException(inner) => Error::RetryableException(inner),
            crate::error::GetChunkErrorKind::ServiceInternalException(inner) => Error::ServiceInternalException(inner),
            crate::error::GetChunkErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::GetChunkErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetObjectMetadataError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetObjectMetadataError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetObjectMetadataError> for Error {
    fn from(err: crate::error::GetObjectMetadataError) -> Self {
        match err.kind {
            crate::error::GetObjectMetadataErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::GetObjectMetadataErrorKind::IllegalArgumentException(inner) => Error::IllegalArgumentException(inner),
            crate::error::GetObjectMetadataErrorKind::KmsInvalidKeyUsageException(inner) => Error::KmsInvalidKeyUsageException(inner),
            crate::error::GetObjectMetadataErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetObjectMetadataErrorKind::RetryableException(inner) => Error::RetryableException(inner),
            crate::error::GetObjectMetadataErrorKind::ServiceInternalException(inner) => Error::ServiceInternalException(inner),
            crate::error::GetObjectMetadataErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
            crate::error::GetObjectMetadataErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::GetObjectMetadataErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListChunksError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListChunksError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListChunksError> for Error {
    fn from(err: crate::error::ListChunksError) -> Self {
        match err.kind {
            crate::error::ListChunksErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ListChunksErrorKind::IllegalArgumentException(inner) => Error::IllegalArgumentException(inner),
            crate::error::ListChunksErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::ListChunksErrorKind::RetryableException(inner) => Error::RetryableException(inner),
            crate::error::ListChunksErrorKind::ServiceInternalException(inner) => Error::ServiceInternalException(inner),
            crate::error::ListChunksErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
            crate::error::ListChunksErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListObjectsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListObjectsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListObjectsError> for Error {
    fn from(err: crate::error::ListObjectsError) -> Self {
        match err.kind {
            crate::error::ListObjectsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::ListObjectsErrorKind::IllegalArgumentException(inner) => Error::IllegalArgumentException(inner),
            crate::error::ListObjectsErrorKind::KmsInvalidKeyUsageException(inner) => Error::KmsInvalidKeyUsageException(inner),
            crate::error::ListObjectsErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::ListObjectsErrorKind::RetryableException(inner) => Error::RetryableException(inner),
            crate::error::ListObjectsErrorKind::ServiceInternalException(inner) => Error::ServiceInternalException(inner),
            crate::error::ListObjectsErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
            crate::error::ListObjectsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::ListObjectsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::NotifyObjectCompleteError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::NotifyObjectCompleteError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::NotifyObjectCompleteError> for Error {
    fn from(err: crate::error::NotifyObjectCompleteError) -> Self {
        match err.kind {
            crate::error::NotifyObjectCompleteErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::NotifyObjectCompleteErrorKind::IllegalArgumentException(inner) => Error::IllegalArgumentException(inner),
            crate::error::NotifyObjectCompleteErrorKind::KmsInvalidKeyUsageException(inner) => Error::KmsInvalidKeyUsageException(inner),
            crate::error::NotifyObjectCompleteErrorKind::NotReadableInputStreamException(inner) => Error::NotReadableInputStreamException(inner),
            crate::error::NotifyObjectCompleteErrorKind::RetryableException(inner) => Error::RetryableException(inner),
            crate::error::NotifyObjectCompleteErrorKind::ServiceInternalException(inner) => Error::ServiceInternalException(inner),
            crate::error::NotifyObjectCompleteErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
            crate::error::NotifyObjectCompleteErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::NotifyObjectCompleteErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutChunkError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PutChunkError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::PutChunkError> for Error {
    fn from(err: crate::error::PutChunkError) -> Self {
        match err.kind {
            crate::error::PutChunkErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::PutChunkErrorKind::IllegalArgumentException(inner) => Error::IllegalArgumentException(inner),
            crate::error::PutChunkErrorKind::KmsInvalidKeyUsageException(inner) => Error::KmsInvalidKeyUsageException(inner),
            crate::error::PutChunkErrorKind::NotReadableInputStreamException(inner) => Error::NotReadableInputStreamException(inner),
            crate::error::PutChunkErrorKind::RetryableException(inner) => Error::RetryableException(inner),
            crate::error::PutChunkErrorKind::ServiceInternalException(inner) => Error::ServiceInternalException(inner),
            crate::error::PutChunkErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
            crate::error::PutChunkErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::PutChunkErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutObjectError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PutObjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::PutObjectError> for Error {
    fn from(err: crate::error::PutObjectError) -> Self {
        match err.kind {
            crate::error::PutObjectErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::PutObjectErrorKind::IllegalArgumentException(inner) => Error::IllegalArgumentException(inner),
            crate::error::PutObjectErrorKind::KmsInvalidKeyUsageException(inner) => Error::KmsInvalidKeyUsageException(inner),
            crate::error::PutObjectErrorKind::NotReadableInputStreamException(inner) => Error::NotReadableInputStreamException(inner),
            crate::error::PutObjectErrorKind::RetryableException(inner) => Error::RetryableException(inner),
            crate::error::PutObjectErrorKind::ServiceInternalException(inner) => Error::ServiceInternalException(inner),
            crate::error::PutObjectErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
            crate::error::PutObjectErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::PutObjectErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::StartObjectError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::StartObjectError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::StartObjectError> for Error {
    fn from(err: crate::error::StartObjectError) -> Self {
        match err.kind {
            crate::error::StartObjectErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::error::StartObjectErrorKind::DataAlreadyExistsException(inner) => Error::DataAlreadyExistsException(inner),
            crate::error::StartObjectErrorKind::IllegalArgumentException(inner) => Error::IllegalArgumentException(inner),
            crate::error::StartObjectErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::StartObjectErrorKind::RetryableException(inner) => Error::RetryableException(inner),
            crate::error::StartObjectErrorKind::ServiceInternalException(inner) => Error::ServiceInternalException(inner),
            crate::error::StartObjectErrorKind::ServiceUnavailableException(inner) => Error::ServiceUnavailableException(inner),
            crate::error::StartObjectErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::error::StartObjectErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl std::error::Error for Error {}


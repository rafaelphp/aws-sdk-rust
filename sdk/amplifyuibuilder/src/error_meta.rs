// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>An internal error has occurred. Please retry your request.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>An invalid or out-of-range value was supplied for the input parameter.</p>
    InvalidParameterException(crate::error::InvalidParameterException),
    /// <p>The resource specified in the request conflicts with an existing resource.</p>
    ResourceConflictException(crate::error::ResourceConflictException),
    /// <p>The requested resource does not exist, or access was denied.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>You exceeded your service quota. Service quotas, also referred to as limits, are the maximum number of service resources or operations for your Amazon Web Services account. </p>
    ServiceQuotaExceededException(crate::error::ServiceQuotaExceededException),
    /// <p>You don't have permission to perform this operation.</p>
    UnauthorizedException(crate::error::UnauthorizedException),
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
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::InvalidParameterException(inner) => inner.fmt(f),
            Error::ResourceConflictException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::UnauthorizedException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f)
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateComponentError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateComponentError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateComponentError> for Error {
    fn from(err: crate::error::CreateComponentError) -> Self {
        match err.kind {
            crate::error::CreateComponentErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::CreateComponentErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::CreateComponentErrorKind::ResourceConflictException(inner) => Error::ResourceConflictException(inner),
            crate::error::CreateComponentErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::error::CreateComponentErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateFormError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateFormError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateFormError> for Error {
    fn from(err: crate::error::CreateFormError) -> Self {
        match err.kind {
            crate::error::CreateFormErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::CreateFormErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::CreateFormErrorKind::ResourceConflictException(inner) => Error::ResourceConflictException(inner),
            crate::error::CreateFormErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::error::CreateFormErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateThemeError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateThemeError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreateThemeError> for Error {
    fn from(err: crate::error::CreateThemeError) -> Self {
        match err.kind {
            crate::error::CreateThemeErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::CreateThemeErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::CreateThemeErrorKind::ResourceConflictException(inner) => Error::ResourceConflictException(inner),
            crate::error::CreateThemeErrorKind::ServiceQuotaExceededException(inner) => Error::ServiceQuotaExceededException(inner),
            crate::error::CreateThemeErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteComponentError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteComponentError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteComponentError> for Error {
    fn from(err: crate::error::DeleteComponentError) -> Self {
        match err.kind {
            crate::error::DeleteComponentErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::DeleteComponentErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::DeleteComponentErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeleteComponentErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteFormError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteFormError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteFormError> for Error {
    fn from(err: crate::error::DeleteFormError) -> Self {
        match err.kind {
            crate::error::DeleteFormErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::DeleteFormErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::DeleteFormErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeleteFormErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteThemeError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteThemeError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeleteThemeError> for Error {
    fn from(err: crate::error::DeleteThemeError) -> Self {
        match err.kind {
            crate::error::DeleteThemeErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::DeleteThemeErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::DeleteThemeErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::DeleteThemeErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ExchangeCodeForTokenError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ExchangeCodeForTokenError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ExchangeCodeForTokenError> for Error {
    fn from(err: crate::error::ExchangeCodeForTokenError) -> Self {
        match err.kind {
            crate::error::ExchangeCodeForTokenErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::ExchangeCodeForTokenErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ExportComponentsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ExportComponentsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ExportComponentsError> for Error {
    fn from(err: crate::error::ExportComponentsError) -> Self {
        match err.kind {
            crate::error::ExportComponentsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::ExportComponentsErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::ExportComponentsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ExportFormsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ExportFormsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ExportFormsError> for Error {
    fn from(err: crate::error::ExportFormsError) -> Self {
        match err.kind {
            crate::error::ExportFormsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::ExportFormsErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::ExportFormsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ExportThemesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ExportThemesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ExportThemesError> for Error {
    fn from(err: crate::error::ExportThemesError) -> Self {
        match err.kind {
            crate::error::ExportThemesErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::ExportThemesErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::ExportThemesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetComponentError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetComponentError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetComponentError> for Error {
    fn from(err: crate::error::GetComponentError) -> Self {
        match err.kind {
            crate::error::GetComponentErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::GetComponentErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::GetComponentErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetComponentErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetFormError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetFormError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetFormError> for Error {
    fn from(err: crate::error::GetFormError) -> Self {
        match err.kind {
            crate::error::GetFormErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::GetFormErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::GetFormErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetFormErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetMetadataError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetMetadataError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetMetadataError> for Error {
    fn from(err: crate::error::GetMetadataError) -> Self {
        match err.kind {
            crate::error::GetMetadataErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::GetMetadataErrorKind::UnauthorizedException(inner) => Error::UnauthorizedException(inner),
            crate::error::GetMetadataErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetThemeError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetThemeError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetThemeError> for Error {
    fn from(err: crate::error::GetThemeError) -> Self {
        match err.kind {
            crate::error::GetThemeErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::GetThemeErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::GetThemeErrorKind::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::error::GetThemeErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListComponentsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListComponentsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListComponentsError> for Error {
    fn from(err: crate::error::ListComponentsError) -> Self {
        match err.kind {
            crate::error::ListComponentsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::ListComponentsErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::ListComponentsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListFormsError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListFormsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListFormsError> for Error {
    fn from(err: crate::error::ListFormsError) -> Self {
        match err.kind {
            crate::error::ListFormsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::ListFormsErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::ListFormsErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListThemesError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListThemesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListThemesError> for Error {
    fn from(err: crate::error::ListThemesError) -> Self {
        match err.kind {
            crate::error::ListThemesErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::ListThemesErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::ListThemesErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutMetadataFlagError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PutMetadataFlagError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::PutMetadataFlagError> for Error {
    fn from(err: crate::error::PutMetadataFlagError) -> Self {
        match err.kind {
            crate::error::PutMetadataFlagErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::PutMetadataFlagErrorKind::UnauthorizedException(inner) => Error::UnauthorizedException(inner),
            crate::error::PutMetadataFlagErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RefreshTokenError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::RefreshTokenError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::RefreshTokenError> for Error {
    fn from(err: crate::error::RefreshTokenError) -> Self {
        match err.kind {
            crate::error::RefreshTokenErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::RefreshTokenErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateComponentError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateComponentError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateComponentError> for Error {
    fn from(err: crate::error::UpdateComponentError) -> Self {
        match err.kind {
            crate::error::UpdateComponentErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::UpdateComponentErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::UpdateComponentErrorKind::ResourceConflictException(inner) => Error::ResourceConflictException(inner),
            crate::error::UpdateComponentErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateFormError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateFormError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateFormError> for Error {
    fn from(err: crate::error::UpdateFormError) -> Self {
        match err.kind {
            crate::error::UpdateFormErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::UpdateFormErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::UpdateFormErrorKind::ResourceConflictException(inner) => Error::ResourceConflictException(inner),
            crate::error::UpdateFormErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateThemeError, R>> for Error where R: Send + Sync + std::fmt::Debug + 'static {
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateThemeError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::UpdateThemeError> for Error {
    fn from(err: crate::error::UpdateThemeError) -> Self {
        match err.kind {
            crate::error::UpdateThemeErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::error::UpdateThemeErrorKind::InvalidParameterException(inner) => Error::InvalidParameterException(inner),
            crate::error::UpdateThemeErrorKind::ResourceConflictException(inner) => Error::ResourceConflictException(inner),
            crate::error::UpdateThemeErrorKind::Unhandled(inner) => Error::Unhandled(crate::error::Unhandled::new(inner.into())),
        }
    }
}
impl std::error::Error for Error {}


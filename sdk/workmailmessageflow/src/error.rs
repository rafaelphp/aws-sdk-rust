// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `PutRawMessageContent` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct PutRawMessageContentError {
    /// Kind of error that occurred.
    pub kind: PutRawMessageContentErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for PutRawMessageContentError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: PutRawMessageContentErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `PutRawMessageContent` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum PutRawMessageContentErrorKind {
    /// <p>WorkMail could not access the updated email content. Possible reasons:</p>
    /// <ul>
    /// <li> <p>You made the request in a region other than your S3 bucket region.</p> </li>
    /// <li> <p>The <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-owner-condition.html">S3 bucket owner</a> is not the same as the calling AWS account.</p> </li>
    /// <li> <p>You have an incomplete or missing S3 bucket policy. For more information about policies, see <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/update-with-lambda.html"> Updating message content with AWS Lambda </a> in the <i>WorkMail Administrator Guide</i>.</p> </li>
    /// </ul>
    InvalidContentLocation(crate::error::InvalidContentLocation),
    /// <p>The requested email is not eligible for update. This is usually the case for a redirected email.</p>
    MessageFrozen(crate::error::MessageFrozen),
    /// <p>The requested email could not be updated due to an error in the MIME content. Check the error message for more information about what caused the error.</p>
    MessageRejected(crate::error::MessageRejected),
    /// <p>The requested email message is not found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for PutRawMessageContentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            PutRawMessageContentErrorKind::InvalidContentLocation(_inner) => _inner.fmt(f),
            PutRawMessageContentErrorKind::MessageFrozen(_inner) => _inner.fmt(f),
            PutRawMessageContentErrorKind::MessageRejected(_inner) => _inner.fmt(f),
            PutRawMessageContentErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            PutRawMessageContentErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for PutRawMessageContentError {
    fn code(&self) -> Option<&str> {
        PutRawMessageContentError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl PutRawMessageContentError {
    /// Creates a new `PutRawMessageContentError`.
    pub fn new(kind: PutRawMessageContentErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `PutRawMessageContentError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: PutRawMessageContentErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
            meta: Default::default(),
        }
    }

    /// Creates the `PutRawMessageContentError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: PutRawMessageContentErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `PutRawMessageContentErrorKind::InvalidContentLocation`.
    pub fn is_invalid_content_location(&self) -> bool {
        matches!(
            &self.kind,
            PutRawMessageContentErrorKind::InvalidContentLocation(_)
        )
    }
    /// Returns `true` if the error kind is `PutRawMessageContentErrorKind::MessageFrozen`.
    pub fn is_message_frozen(&self) -> bool {
        matches!(&self.kind, PutRawMessageContentErrorKind::MessageFrozen(_))
    }
    /// Returns `true` if the error kind is `PutRawMessageContentErrorKind::MessageRejected`.
    pub fn is_message_rejected(&self) -> bool {
        matches!(
            &self.kind,
            PutRawMessageContentErrorKind::MessageRejected(_)
        )
    }
    /// Returns `true` if the error kind is `PutRawMessageContentErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            PutRawMessageContentErrorKind::ResourceNotFoundException(_)
        )
    }
}
impl std::error::Error for PutRawMessageContentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            PutRawMessageContentErrorKind::InvalidContentLocation(_inner) => Some(_inner),
            PutRawMessageContentErrorKind::MessageFrozen(_inner) => Some(_inner),
            PutRawMessageContentErrorKind::MessageRejected(_inner) => Some(_inner),
            PutRawMessageContentErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            PutRawMessageContentErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

/// <p>The requested email message is not found.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ResourceNotFoundException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl ResourceNotFoundException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceNotFoundException")?;
        if let Some(inner_1) = &self.message {
            {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for ResourceNotFoundException {}
/// See [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
pub mod resource_not_found_exception {

    /// A builder for [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
        pub fn build(self) -> crate::error::ResourceNotFoundException {
            crate::error::ResourceNotFoundException {
                message: self.message,
            }
        }
    }
}
impl ResourceNotFoundException {
    /// Creates a new builder-style object to manufacture [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
    pub fn builder() -> crate::error::resource_not_found_exception::Builder {
        crate::error::resource_not_found_exception::Builder::default()
    }
}

/// <p>The requested email could not be updated due to an error in the MIME content. Check the error message for more information about what caused the error.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct MessageRejected {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl MessageRejected {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for MessageRejected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MessageRejected")?;
        if let Some(inner_2) = &self.message {
            {
                write!(f, ": {}", inner_2)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for MessageRejected {}
/// See [`MessageRejected`](crate::error::MessageRejected).
pub mod message_rejected {

    /// A builder for [`MessageRejected`](crate::error::MessageRejected).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`MessageRejected`](crate::error::MessageRejected).
        pub fn build(self) -> crate::error::MessageRejected {
            crate::error::MessageRejected {
                message: self.message,
            }
        }
    }
}
impl MessageRejected {
    /// Creates a new builder-style object to manufacture [`MessageRejected`](crate::error::MessageRejected).
    pub fn builder() -> crate::error::message_rejected::Builder {
        crate::error::message_rejected::Builder::default()
    }
}

/// <p>The requested email is not eligible for update. This is usually the case for a redirected email.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct MessageFrozen {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl MessageFrozen {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for MessageFrozen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MessageFrozen")?;
        if let Some(inner_3) = &self.message {
            {
                write!(f, ": {}", inner_3)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for MessageFrozen {}
/// See [`MessageFrozen`](crate::error::MessageFrozen).
pub mod message_frozen {

    /// A builder for [`MessageFrozen`](crate::error::MessageFrozen).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`MessageFrozen`](crate::error::MessageFrozen).
        pub fn build(self) -> crate::error::MessageFrozen {
            crate::error::MessageFrozen {
                message: self.message,
            }
        }
    }
}
impl MessageFrozen {
    /// Creates a new builder-style object to manufacture [`MessageFrozen`](crate::error::MessageFrozen).
    pub fn builder() -> crate::error::message_frozen::Builder {
        crate::error::message_frozen::Builder::default()
    }
}

/// <p>WorkMail could not access the updated email content. Possible reasons:</p>
/// <ul>
/// <li> <p>You made the request in a region other than your S3 bucket region.</p> </li>
/// <li> <p>The <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-owner-condition.html">S3 bucket owner</a> is not the same as the calling AWS account.</p> </li>
/// <li> <p>You have an incomplete or missing S3 bucket policy. For more information about policies, see <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/update-with-lambda.html"> Updating message content with AWS Lambda </a> in the <i>WorkMail Administrator Guide</i>.</p> </li>
/// </ul>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct InvalidContentLocation {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl InvalidContentLocation {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InvalidContentLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidContentLocation")?;
        if let Some(inner_4) = &self.message {
            {
                write!(f, ": {}", inner_4)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for InvalidContentLocation {}
/// See [`InvalidContentLocation`](crate::error::InvalidContentLocation).
pub mod invalid_content_location {

    /// A builder for [`InvalidContentLocation`](crate::error::InvalidContentLocation).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InvalidContentLocation`](crate::error::InvalidContentLocation).
        pub fn build(self) -> crate::error::InvalidContentLocation {
            crate::error::InvalidContentLocation {
                message: self.message,
            }
        }
    }
}
impl InvalidContentLocation {
    /// Creates a new builder-style object to manufacture [`InvalidContentLocation`](crate::error::InvalidContentLocation).
    pub fn builder() -> crate::error::invalid_content_location::Builder {
        crate::error::invalid_content_location::Builder::default()
    }
}

/// Error type for the `GetRawMessageContent` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetRawMessageContentError {
    /// Kind of error that occurred.
    pub kind: GetRawMessageContentErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
impl aws_smithy_http::result::CreateUnhandledError for GetRawMessageContentError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: GetRawMessageContentErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default(),
        }
    }
}
/// Types of errors that can occur for the `GetRawMessageContent` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetRawMessageContentErrorKind {
    /// <p>The requested email message is not found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for GetRawMessageContentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetRawMessageContentErrorKind::ResourceNotFoundException(_inner) => _inner.fmt(f),
            GetRawMessageContentErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GetRawMessageContentError {
    fn code(&self) -> Option<&str> {
        GetRawMessageContentError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetRawMessageContentError {
    /// Creates a new `GetRawMessageContentError`.
    pub fn new(kind: GetRawMessageContentErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `GetRawMessageContentError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GetRawMessageContentErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
            meta: Default::default(),
        }
    }

    /// Creates the `GetRawMessageContentError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GetRawMessageContentErrorKind::Unhandled(crate::error::Unhandled::new(
                err.into(),
            )),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `GetRawMessageContentErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetRawMessageContentErrorKind::ResourceNotFoundException(_)
        )
    }
}
impl std::error::Error for GetRawMessageContentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetRawMessageContentErrorKind::ResourceNotFoundException(_inner) => Some(_inner),
            GetRawMessageContentErrorKind::Unhandled(_inner) => Some(_inner),
        }
    }
}

///
/// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
///
/// When logging an error from the SDK, it is recommended that you either wrap the error in
/// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
/// error reporter library that visits the error's cause/source chain, or call
/// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
///
#[derive(Debug)]
pub struct Unhandled {
    source: Box<dyn std::error::Error + Send + Sync + 'static>,
}
impl Unhandled {
    #[allow(unused)]
    pub(crate) fn new(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self { source }
    }
}
impl std::fmt::Display for Unhandled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "unhandled error")
    }
}
impl std::error::Error for Unhandled {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.source.as_ref() as _)
    }
}

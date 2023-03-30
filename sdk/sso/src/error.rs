// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `Logout` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct LogoutError {
    /// Kind of error that occurred.
                    pub kind: LogoutErrorKind,
                    /// Additional metadata about the error, including error code, message, and request ID.
                    pub (crate) meta: aws_smithy_types::Error
}
impl aws_smithy_http::result::CreateUnhandledError for LogoutError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: LogoutErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default()
        }
    }
}
/// Types of errors that can occur for the `Logout` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum LogoutErrorKind {
    /// <p>Indicates that a problem occurred with the input to the request. For example, a required parameter might be missing or out of range.</p>
    InvalidRequestException(crate::error::InvalidRequestException),
    /// <p>Indicates that the request is being made too frequently and is more than what the server can handle.</p>
    TooManyRequestsException(crate::error::TooManyRequestsException),
    /// <p>Indicates that the request is not authorized. This can happen due to an invalid access token in the request.</p>
    UnauthorizedException(crate::error::UnauthorizedException),
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
impl std::fmt::Display for LogoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            LogoutErrorKind::InvalidRequestException(_inner) =>
            _inner.fmt(f)
            ,
            LogoutErrorKind::TooManyRequestsException(_inner) =>
            _inner.fmt(f)
            ,
            LogoutErrorKind::UnauthorizedException(_inner) =>
            _inner.fmt(f)
            ,
            LogoutErrorKind::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for LogoutError {
    fn code(&self) -> Option<&str> {
        LogoutError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl LogoutError {
    /// Creates a new `LogoutError`.
                    pub fn new(kind: LogoutErrorKind, meta: aws_smithy_types::Error) -> Self {
                        Self { kind, meta }
                    }
    
                    /// Creates the `LogoutError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self {
                            kind: LogoutErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
                            meta: Default::default()
                        }
                    }
    
                    /// Creates the `LogoutError::Unhandled` variant from a `aws_smithy_types::Error`.
                    pub fn generic(err: aws_smithy_types::Error) -> Self {
                        Self {
                            meta: err.clone(),
                            kind: LogoutErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
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
    /// Returns `true` if the error kind is `LogoutErrorKind::InvalidRequestException`.
    pub fn is_invalid_request_exception(&self) -> bool {
        matches!(&self.kind, LogoutErrorKind::InvalidRequestException(_))
    }
    /// Returns `true` if the error kind is `LogoutErrorKind::TooManyRequestsException`.
    pub fn is_too_many_requests_exception(&self) -> bool {
        matches!(&self.kind, LogoutErrorKind::TooManyRequestsException(_))
    }
    /// Returns `true` if the error kind is `LogoutErrorKind::UnauthorizedException`.
    pub fn is_unauthorized_exception(&self) -> bool {
        matches!(&self.kind, LogoutErrorKind::UnauthorizedException(_))
    }
}
impl std::error::Error for LogoutError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            LogoutErrorKind::InvalidRequestException(_inner) =>
            Some(_inner)
            ,
            LogoutErrorKind::TooManyRequestsException(_inner) =>
            Some(_inner)
            ,
            LogoutErrorKind::UnauthorizedException(_inner) =>
            Some(_inner)
            ,
            LogoutErrorKind::Unhandled(_inner) => {
                Some(_inner)
            }
        }
    }
}

/// <p>Indicates that the request is not authorized. This can happen due to an invalid access token in the request.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UnauthorizedException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl UnauthorizedException {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for UnauthorizedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnauthorizedException")?;
        if let Some(inner_1) = &self.message {
             {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for UnauthorizedException {}
/// See [`UnauthorizedException`](crate::error::UnauthorizedException).
pub mod unauthorized_exception {
    
    /// A builder for [`UnauthorizedException`](crate::error::UnauthorizedException).
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
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`UnauthorizedException`](crate::error::UnauthorizedException).
        pub fn build(self) -> crate::error::UnauthorizedException {
            crate::error::UnauthorizedException {
                message: self.message
                ,
            }
        }
    }
    
    
}
impl UnauthorizedException {
    /// Creates a new builder-style object to manufacture [`UnauthorizedException`](crate::error::UnauthorizedException).
    pub fn builder() -> crate::error::unauthorized_exception::Builder {
        crate::error::unauthorized_exception::Builder::default()
    }
}

/// <p>Indicates that the request is being made too frequently and is more than what the server can handle.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct TooManyRequestsException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl TooManyRequestsException {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for TooManyRequestsException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TooManyRequestsException")?;
        if let Some(inner_2) = &self.message {
             {
                write!(f, ": {}", inner_2)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for TooManyRequestsException {}
/// See [`TooManyRequestsException`](crate::error::TooManyRequestsException).
pub mod too_many_requests_exception {
    
    /// A builder for [`TooManyRequestsException`](crate::error::TooManyRequestsException).
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
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`TooManyRequestsException`](crate::error::TooManyRequestsException).
        pub fn build(self) -> crate::error::TooManyRequestsException {
            crate::error::TooManyRequestsException {
                message: self.message
                ,
            }
        }
    }
    
    
}
impl TooManyRequestsException {
    /// Creates a new builder-style object to manufacture [`TooManyRequestsException`](crate::error::TooManyRequestsException).
    pub fn builder() -> crate::error::too_many_requests_exception::Builder {
        crate::error::too_many_requests_exception::Builder::default()
    }
}

/// <p>Indicates that a problem occurred with the input to the request. For example, a required parameter might be missing or out of range.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct InvalidRequestException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl InvalidRequestException {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for InvalidRequestException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidRequestException")?;
        if let Some(inner_3) = &self.message {
             {
                write!(f, ": {}", inner_3)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for InvalidRequestException {}
/// See [`InvalidRequestException`](crate::error::InvalidRequestException).
pub mod invalid_request_exception {
    
    /// A builder for [`InvalidRequestException`](crate::error::InvalidRequestException).
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
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`InvalidRequestException`](crate::error::InvalidRequestException).
        pub fn build(self) -> crate::error::InvalidRequestException {
            crate::error::InvalidRequestException {
                message: self.message
                ,
            }
        }
    }
    
    
}
impl InvalidRequestException {
    /// Creates a new builder-style object to manufacture [`InvalidRequestException`](crate::error::InvalidRequestException).
    pub fn builder() -> crate::error::invalid_request_exception::Builder {
        crate::error::invalid_request_exception::Builder::default()
    }
}

/// Error type for the `ListAccounts` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct ListAccountsError {
    /// Kind of error that occurred.
                    pub kind: ListAccountsErrorKind,
                    /// Additional metadata about the error, including error code, message, and request ID.
                    pub (crate) meta: aws_smithy_types::Error
}
impl aws_smithy_http::result::CreateUnhandledError for ListAccountsError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: ListAccountsErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default()
        }
    }
}
/// Types of errors that can occur for the `ListAccounts` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum ListAccountsErrorKind {
    /// <p>Indicates that a problem occurred with the input to the request. For example, a required parameter might be missing or out of range.</p>
    InvalidRequestException(crate::error::InvalidRequestException),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>Indicates that the request is being made too frequently and is more than what the server can handle.</p>
    TooManyRequestsException(crate::error::TooManyRequestsException),
    /// <p>Indicates that the request is not authorized. This can happen due to an invalid access token in the request.</p>
    UnauthorizedException(crate::error::UnauthorizedException),
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
impl std::fmt::Display for ListAccountsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ListAccountsErrorKind::InvalidRequestException(_inner) =>
            _inner.fmt(f)
            ,
            ListAccountsErrorKind::ResourceNotFoundException(_inner) =>
            _inner.fmt(f)
            ,
            ListAccountsErrorKind::TooManyRequestsException(_inner) =>
            _inner.fmt(f)
            ,
            ListAccountsErrorKind::UnauthorizedException(_inner) =>
            _inner.fmt(f)
            ,
            ListAccountsErrorKind::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for ListAccountsError {
    fn code(&self) -> Option<&str> {
        ListAccountsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl ListAccountsError {
    /// Creates a new `ListAccountsError`.
                    pub fn new(kind: ListAccountsErrorKind, meta: aws_smithy_types::Error) -> Self {
                        Self { kind, meta }
                    }
    
                    /// Creates the `ListAccountsError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self {
                            kind: ListAccountsErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
                            meta: Default::default()
                        }
                    }
    
                    /// Creates the `ListAccountsError::Unhandled` variant from a `aws_smithy_types::Error`.
                    pub fn generic(err: aws_smithy_types::Error) -> Self {
                        Self {
                            meta: err.clone(),
                            kind: ListAccountsErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
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
    /// Returns `true` if the error kind is `ListAccountsErrorKind::InvalidRequestException`.
    pub fn is_invalid_request_exception(&self) -> bool {
        matches!(&self.kind, ListAccountsErrorKind::InvalidRequestException(_))
    }
    /// Returns `true` if the error kind is `ListAccountsErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(&self.kind, ListAccountsErrorKind::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `ListAccountsErrorKind::TooManyRequestsException`.
    pub fn is_too_many_requests_exception(&self) -> bool {
        matches!(&self.kind, ListAccountsErrorKind::TooManyRequestsException(_))
    }
    /// Returns `true` if the error kind is `ListAccountsErrorKind::UnauthorizedException`.
    pub fn is_unauthorized_exception(&self) -> bool {
        matches!(&self.kind, ListAccountsErrorKind::UnauthorizedException(_))
    }
}
impl std::error::Error for ListAccountsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ListAccountsErrorKind::InvalidRequestException(_inner) =>
            Some(_inner)
            ,
            ListAccountsErrorKind::ResourceNotFoundException(_inner) =>
            Some(_inner)
            ,
            ListAccountsErrorKind::TooManyRequestsException(_inner) =>
            Some(_inner)
            ,
            ListAccountsErrorKind::UnauthorizedException(_inner) =>
            Some(_inner)
            ,
            ListAccountsErrorKind::Unhandled(_inner) => {
                Some(_inner)
            }
        }
    }
}

/// <p>The specified resource doesn't exist.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ResourceNotFoundException  {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl ResourceNotFoundException {
    /// Returns the error message.
                        pub fn message(&self) -> std::option::Option<& str> { self.message.as_deref() }
}
impl std::fmt::Display for ResourceNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceNotFoundException")?;
        if let Some(inner_4) = &self.message {
             {
                write!(f, ": {}", inner_4)?;
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
            self.message = input; self
        }
        /// Consumes the builder and constructs a [`ResourceNotFoundException`](crate::error::ResourceNotFoundException).
        pub fn build(self) -> crate::error::ResourceNotFoundException {
            crate::error::ResourceNotFoundException {
                message: self.message
                ,
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

/// Error type for the `ListAccountRoles` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct ListAccountRolesError {
    /// Kind of error that occurred.
                    pub kind: ListAccountRolesErrorKind,
                    /// Additional metadata about the error, including error code, message, and request ID.
                    pub (crate) meta: aws_smithy_types::Error
}
impl aws_smithy_http::result::CreateUnhandledError for ListAccountRolesError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: ListAccountRolesErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default()
        }
    }
}
/// Types of errors that can occur for the `ListAccountRoles` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum ListAccountRolesErrorKind {
    /// <p>Indicates that a problem occurred with the input to the request. For example, a required parameter might be missing or out of range.</p>
    InvalidRequestException(crate::error::InvalidRequestException),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>Indicates that the request is being made too frequently and is more than what the server can handle.</p>
    TooManyRequestsException(crate::error::TooManyRequestsException),
    /// <p>Indicates that the request is not authorized. This can happen due to an invalid access token in the request.</p>
    UnauthorizedException(crate::error::UnauthorizedException),
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
impl std::fmt::Display for ListAccountRolesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ListAccountRolesErrorKind::InvalidRequestException(_inner) =>
            _inner.fmt(f)
            ,
            ListAccountRolesErrorKind::ResourceNotFoundException(_inner) =>
            _inner.fmt(f)
            ,
            ListAccountRolesErrorKind::TooManyRequestsException(_inner) =>
            _inner.fmt(f)
            ,
            ListAccountRolesErrorKind::UnauthorizedException(_inner) =>
            _inner.fmt(f)
            ,
            ListAccountRolesErrorKind::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for ListAccountRolesError {
    fn code(&self) -> Option<&str> {
        ListAccountRolesError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl ListAccountRolesError {
    /// Creates a new `ListAccountRolesError`.
                    pub fn new(kind: ListAccountRolesErrorKind, meta: aws_smithy_types::Error) -> Self {
                        Self { kind, meta }
                    }
    
                    /// Creates the `ListAccountRolesError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self {
                            kind: ListAccountRolesErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
                            meta: Default::default()
                        }
                    }
    
                    /// Creates the `ListAccountRolesError::Unhandled` variant from a `aws_smithy_types::Error`.
                    pub fn generic(err: aws_smithy_types::Error) -> Self {
                        Self {
                            meta: err.clone(),
                            kind: ListAccountRolesErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
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
    /// Returns `true` if the error kind is `ListAccountRolesErrorKind::InvalidRequestException`.
    pub fn is_invalid_request_exception(&self) -> bool {
        matches!(&self.kind, ListAccountRolesErrorKind::InvalidRequestException(_))
    }
    /// Returns `true` if the error kind is `ListAccountRolesErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(&self.kind, ListAccountRolesErrorKind::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `ListAccountRolesErrorKind::TooManyRequestsException`.
    pub fn is_too_many_requests_exception(&self) -> bool {
        matches!(&self.kind, ListAccountRolesErrorKind::TooManyRequestsException(_))
    }
    /// Returns `true` if the error kind is `ListAccountRolesErrorKind::UnauthorizedException`.
    pub fn is_unauthorized_exception(&self) -> bool {
        matches!(&self.kind, ListAccountRolesErrorKind::UnauthorizedException(_))
    }
}
impl std::error::Error for ListAccountRolesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ListAccountRolesErrorKind::InvalidRequestException(_inner) =>
            Some(_inner)
            ,
            ListAccountRolesErrorKind::ResourceNotFoundException(_inner) =>
            Some(_inner)
            ,
            ListAccountRolesErrorKind::TooManyRequestsException(_inner) =>
            Some(_inner)
            ,
            ListAccountRolesErrorKind::UnauthorizedException(_inner) =>
            Some(_inner)
            ,
            ListAccountRolesErrorKind::Unhandled(_inner) => {
                Some(_inner)
            }
        }
    }
}

/// Error type for the `GetRoleCredentials` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetRoleCredentialsError {
    /// Kind of error that occurred.
                    pub kind: GetRoleCredentialsErrorKind,
                    /// Additional metadata about the error, including error code, message, and request ID.
                    pub (crate) meta: aws_smithy_types::Error
}
impl aws_smithy_http::result::CreateUnhandledError for GetRoleCredentialsError {
    fn create_unhandled_error(source: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            kind: GetRoleCredentialsErrorKind::Unhandled(crate::error::Unhandled::new(source)),
            meta: Default::default()
        }
    }
}
/// Types of errors that can occur for the `GetRoleCredentials` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetRoleCredentialsErrorKind {
    /// <p>Indicates that a problem occurred with the input to the request. For example, a required parameter might be missing or out of range.</p>
    InvalidRequestException(crate::error::InvalidRequestException),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>Indicates that the request is being made too frequently and is more than what the server can handle.</p>
    TooManyRequestsException(crate::error::TooManyRequestsException),
    /// <p>Indicates that the request is not authorized. This can happen due to an invalid access token in the request.</p>
    UnauthorizedException(crate::error::UnauthorizedException),
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
impl std::fmt::Display for GetRoleCredentialsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetRoleCredentialsErrorKind::InvalidRequestException(_inner) =>
            _inner.fmt(f)
            ,
            GetRoleCredentialsErrorKind::ResourceNotFoundException(_inner) =>
            _inner.fmt(f)
            ,
            GetRoleCredentialsErrorKind::TooManyRequestsException(_inner) =>
            _inner.fmt(f)
            ,
            GetRoleCredentialsErrorKind::UnauthorizedException(_inner) =>
            _inner.fmt(f)
            ,
            GetRoleCredentialsErrorKind::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GetRoleCredentialsError {
    fn code(&self) -> Option<&str> {
        GetRoleCredentialsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetRoleCredentialsError {
    /// Creates a new `GetRoleCredentialsError`.
                    pub fn new(kind: GetRoleCredentialsErrorKind, meta: aws_smithy_types::Error) -> Self {
                        Self { kind, meta }
                    }
    
                    /// Creates the `GetRoleCredentialsError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self {
                            kind: GetRoleCredentialsErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
                            meta: Default::default()
                        }
                    }
    
                    /// Creates the `GetRoleCredentialsError::Unhandled` variant from a `aws_smithy_types::Error`.
                    pub fn generic(err: aws_smithy_types::Error) -> Self {
                        Self {
                            meta: err.clone(),
                            kind: GetRoleCredentialsErrorKind::Unhandled(crate::error::Unhandled::new(err.into())),
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
    /// Returns `true` if the error kind is `GetRoleCredentialsErrorKind::InvalidRequestException`.
    pub fn is_invalid_request_exception(&self) -> bool {
        matches!(&self.kind, GetRoleCredentialsErrorKind::InvalidRequestException(_))
    }
    /// Returns `true` if the error kind is `GetRoleCredentialsErrorKind::ResourceNotFoundException`.
    pub fn is_resource_not_found_exception(&self) -> bool {
        matches!(&self.kind, GetRoleCredentialsErrorKind::ResourceNotFoundException(_))
    }
    /// Returns `true` if the error kind is `GetRoleCredentialsErrorKind::TooManyRequestsException`.
    pub fn is_too_many_requests_exception(&self) -> bool {
        matches!(&self.kind, GetRoleCredentialsErrorKind::TooManyRequestsException(_))
    }
    /// Returns `true` if the error kind is `GetRoleCredentialsErrorKind::UnauthorizedException`.
    pub fn is_unauthorized_exception(&self) -> bool {
        matches!(&self.kind, GetRoleCredentialsErrorKind::UnauthorizedException(_))
    }
}
impl std::error::Error for GetRoleCredentialsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetRoleCredentialsErrorKind::InvalidRequestException(_inner) =>
            Some(_inner)
            ,
            GetRoleCredentialsErrorKind::ResourceNotFoundException(_inner) =>
            Some(_inner)
            ,
            GetRoleCredentialsErrorKind::TooManyRequestsException(_inner) =>
            Some(_inner)
            ,
            GetRoleCredentialsErrorKind::UnauthorizedException(_inner) =>
            Some(_inner)
            ,
            GetRoleCredentialsErrorKind::Unhandled(_inner) => {
                Some(_inner)
            }
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


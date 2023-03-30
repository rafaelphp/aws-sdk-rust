// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `PutEvents`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_events`](crate::client::Client::put_events).
            ///
            /// See [`crate::client::fluent_builders::PutEvents`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutEvents {
    _private: ()
}
impl PutEvents {
    /// Creates a new builder-style object to manufacture [`PutEventsInput`](crate::input::PutEventsInput).
    pub fn builder() -> crate::input::put_events_input::Builder {
        crate::input::put_events_input::Builder::default()
    }
    /// Creates a new `PutEvents` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutEvents {
                type Output = std::result::Result<crate::output::PutEventsOutput, crate::error::PutEventsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_put_events_error(response)
                     } else {
                        crate::operation_deser::parse_put_events_response(response)
                     }
                }
            }

/// Operation shape for `PutItems`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_items`](crate::client::Client::put_items).
            ///
            /// See [`crate::client::fluent_builders::PutItems`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutItems {
    _private: ()
}
impl PutItems {
    /// Creates a new builder-style object to manufacture [`PutItemsInput`](crate::input::PutItemsInput).
    pub fn builder() -> crate::input::put_items_input::Builder {
        crate::input::put_items_input::Builder::default()
    }
    /// Creates a new `PutItems` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutItems {
                type Output = std::result::Result<crate::output::PutItemsOutput, crate::error::PutItemsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_put_items_error(response)
                     } else {
                        crate::operation_deser::parse_put_items_response(response)
                     }
                }
            }

/// Operation shape for `PutUsers`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_users`](crate::client::Client::put_users).
            ///
            /// See [`crate::client::fluent_builders::PutUsers`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutUsers {
    _private: ()
}
impl PutUsers {
    /// Creates a new builder-style object to manufacture [`PutUsersInput`](crate::input::PutUsersInput).
    pub fn builder() -> crate::input::put_users_input::Builder {
        crate::input::put_users_input::Builder::default()
    }
    /// Creates a new `PutUsers` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutUsers {
                type Output = std::result::Result<crate::output::PutUsersOutput, crate::error::PutUsersError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_put_users_error(response)
                     } else {
                        crate::operation_deser::parse_put_users_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;


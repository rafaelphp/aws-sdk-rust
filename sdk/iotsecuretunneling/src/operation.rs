// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CloseTunnel`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`close_tunnel`](crate::client::Client::close_tunnel).
            ///
            /// See [`crate::client::fluent_builders::CloseTunnel`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CloseTunnel {
    _private: ()
}
impl CloseTunnel {
    /// Creates a new builder-style object to manufacture [`CloseTunnelInput`](crate::input::CloseTunnelInput).
    pub fn builder() -> crate::input::close_tunnel_input::Builder {
        crate::input::close_tunnel_input::Builder::default()
    }
    /// Creates a new `CloseTunnel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CloseTunnel {
                type Output = std::result::Result<crate::output::CloseTunnelOutput, crate::error::CloseTunnelError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_close_tunnel_error(response)
                     } else {
                        crate::operation_deser::parse_close_tunnel_response(response)
                     }
                }
            }

/// Operation shape for `DescribeTunnel`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_tunnel`](crate::client::Client::describe_tunnel).
            ///
            /// See [`crate::client::fluent_builders::DescribeTunnel`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeTunnel {
    _private: ()
}
impl DescribeTunnel {
    /// Creates a new builder-style object to manufacture [`DescribeTunnelInput`](crate::input::DescribeTunnelInput).
    pub fn builder() -> crate::input::describe_tunnel_input::Builder {
        crate::input::describe_tunnel_input::Builder::default()
    }
    /// Creates a new `DescribeTunnel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeTunnel {
                type Output = std::result::Result<crate::output::DescribeTunnelOutput, crate::error::DescribeTunnelError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_tunnel_error(response)
                     } else {
                        crate::operation_deser::parse_describe_tunnel_response(response)
                     }
                }
            }

/// Operation shape for `ListTagsForResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
            ///
            /// See [`crate::client::fluent_builders::ListTagsForResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: ()
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput).
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
                type Output = std::result::Result<crate::output::ListTagsForResourceOutput, crate::error::ListTagsForResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_tags_for_resource_error(response)
                     } else {
                        crate::operation_deser::parse_list_tags_for_resource_response(response)
                     }
                }
            }

/// Operation shape for `ListTunnels`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_tunnels`](crate::client::Client::list_tunnels).
            ///
            /// See [`crate::client::fluent_builders::ListTunnels`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTunnels {
    _private: ()
}
impl ListTunnels {
    /// Creates a new builder-style object to manufacture [`ListTunnelsInput`](crate::input::ListTunnelsInput).
    pub fn builder() -> crate::input::list_tunnels_input::Builder {
        crate::input::list_tunnels_input::Builder::default()
    }
    /// Creates a new `ListTunnels` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTunnels {
                type Output = std::result::Result<crate::output::ListTunnelsOutput, crate::error::ListTunnelsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_tunnels_error(response)
                     } else {
                        crate::operation_deser::parse_list_tunnels_response(response)
                     }
                }
            }

/// Operation shape for `OpenTunnel`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`open_tunnel`](crate::client::Client::open_tunnel).
            ///
            /// See [`crate::client::fluent_builders::OpenTunnel`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct OpenTunnel {
    _private: ()
}
impl OpenTunnel {
    /// Creates a new builder-style object to manufacture [`OpenTunnelInput`](crate::input::OpenTunnelInput).
    pub fn builder() -> crate::input::open_tunnel_input::Builder {
        crate::input::open_tunnel_input::Builder::default()
    }
    /// Creates a new `OpenTunnel` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for OpenTunnel {
                type Output = std::result::Result<crate::output::OpenTunnelOutput, crate::error::OpenTunnelError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_open_tunnel_error(response)
                     } else {
                        crate::operation_deser::parse_open_tunnel_response(response)
                     }
                }
            }

/// Operation shape for `RotateTunnelAccessToken`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`rotate_tunnel_access_token`](crate::client::Client::rotate_tunnel_access_token).
            ///
            /// See [`crate::client::fluent_builders::RotateTunnelAccessToken`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct RotateTunnelAccessToken {
    _private: ()
}
impl RotateTunnelAccessToken {
    /// Creates a new builder-style object to manufacture [`RotateTunnelAccessTokenInput`](crate::input::RotateTunnelAccessTokenInput).
    pub fn builder() -> crate::input::rotate_tunnel_access_token_input::Builder {
        crate::input::rotate_tunnel_access_token_input::Builder::default()
    }
    /// Creates a new `RotateTunnelAccessToken` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RotateTunnelAccessToken {
                type Output = std::result::Result<crate::output::RotateTunnelAccessTokenOutput, crate::error::RotateTunnelAccessTokenError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_rotate_tunnel_access_token_error(response)
                     } else {
                        crate::operation_deser::parse_rotate_tunnel_access_token_response(response)
                     }
                }
            }

/// Operation shape for `TagResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`tag_resource`](crate::client::Client::tag_resource).
            ///
            /// See [`crate::client::fluent_builders::TagResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct TagResource {
    _private: ()
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput).
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
                type Output = std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_tag_resource_error(response)
                     } else {
                        crate::operation_deser::parse_tag_resource_response(response)
                     }
                }
            }

/// Operation shape for `UntagResource`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`untag_resource`](crate::client::Client::untag_resource).
            ///
            /// See [`crate::client::fluent_builders::UntagResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UntagResource {
    _private: ()
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput).
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
                type Output = std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_untag_resource_error(response)
                     } else {
                        crate::operation_deser::parse_untag_resource_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;


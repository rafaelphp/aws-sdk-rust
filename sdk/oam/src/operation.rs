// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateLink`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_link`](crate::client::Client::create_link).
            ///
            /// See [`crate::client::fluent_builders::CreateLink`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateLink {
    _private: ()
}
impl CreateLink {
    /// Creates a new builder-style object to manufacture [`CreateLinkInput`](crate::input::CreateLinkInput).
    pub fn builder() -> crate::input::create_link_input::Builder {
        crate::input::create_link_input::Builder::default()
    }
    /// Creates a new `CreateLink` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateLink {
                type Output = std::result::Result<crate::output::CreateLinkOutput, crate::error::CreateLinkError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_link_error(response)
                     } else {
                        crate::operation_deser::parse_create_link_response(response)
                     }
                }
            }

/// Operation shape for `CreateSink`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_sink`](crate::client::Client::create_sink).
            ///
            /// See [`crate::client::fluent_builders::CreateSink`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateSink {
    _private: ()
}
impl CreateSink {
    /// Creates a new builder-style object to manufacture [`CreateSinkInput`](crate::input::CreateSinkInput).
    pub fn builder() -> crate::input::create_sink_input::Builder {
        crate::input::create_sink_input::Builder::default()
    }
    /// Creates a new `CreateSink` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateSink {
                type Output = std::result::Result<crate::output::CreateSinkOutput, crate::error::CreateSinkError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_sink_error(response)
                     } else {
                        crate::operation_deser::parse_create_sink_response(response)
                     }
                }
            }

/// Operation shape for `DeleteLink`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_link`](crate::client::Client::delete_link).
            ///
            /// See [`crate::client::fluent_builders::DeleteLink`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteLink {
    _private: ()
}
impl DeleteLink {
    /// Creates a new builder-style object to manufacture [`DeleteLinkInput`](crate::input::DeleteLinkInput).
    pub fn builder() -> crate::input::delete_link_input::Builder {
        crate::input::delete_link_input::Builder::default()
    }
    /// Creates a new `DeleteLink` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteLink {
                type Output = std::result::Result<crate::output::DeleteLinkOutput, crate::error::DeleteLinkError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_link_error(response)
                     } else {
                        crate::operation_deser::parse_delete_link_response(response)
                     }
                }
            }

/// Operation shape for `DeleteSink`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_sink`](crate::client::Client::delete_sink).
            ///
            /// See [`crate::client::fluent_builders::DeleteSink`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteSink {
    _private: ()
}
impl DeleteSink {
    /// Creates a new builder-style object to manufacture [`DeleteSinkInput`](crate::input::DeleteSinkInput).
    pub fn builder() -> crate::input::delete_sink_input::Builder {
        crate::input::delete_sink_input::Builder::default()
    }
    /// Creates a new `DeleteSink` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteSink {
                type Output = std::result::Result<crate::output::DeleteSinkOutput, crate::error::DeleteSinkError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_sink_error(response)
                     } else {
                        crate::operation_deser::parse_delete_sink_response(response)
                     }
                }
            }

/// Operation shape for `GetLink`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_link`](crate::client::Client::get_link).
            ///
            /// See [`crate::client::fluent_builders::GetLink`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetLink {
    _private: ()
}
impl GetLink {
    /// Creates a new builder-style object to manufacture [`GetLinkInput`](crate::input::GetLinkInput).
    pub fn builder() -> crate::input::get_link_input::Builder {
        crate::input::get_link_input::Builder::default()
    }
    /// Creates a new `GetLink` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetLink {
                type Output = std::result::Result<crate::output::GetLinkOutput, crate::error::GetLinkError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_link_error(response)
                     } else {
                        crate::operation_deser::parse_get_link_response(response)
                     }
                }
            }

/// Operation shape for `GetSink`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_sink`](crate::client::Client::get_sink).
            ///
            /// See [`crate::client::fluent_builders::GetSink`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetSink {
    _private: ()
}
impl GetSink {
    /// Creates a new builder-style object to manufacture [`GetSinkInput`](crate::input::GetSinkInput).
    pub fn builder() -> crate::input::get_sink_input::Builder {
        crate::input::get_sink_input::Builder::default()
    }
    /// Creates a new `GetSink` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetSink {
                type Output = std::result::Result<crate::output::GetSinkOutput, crate::error::GetSinkError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_sink_error(response)
                     } else {
                        crate::operation_deser::parse_get_sink_response(response)
                     }
                }
            }

/// Operation shape for `GetSinkPolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_sink_policy`](crate::client::Client::get_sink_policy).
            ///
            /// See [`crate::client::fluent_builders::GetSinkPolicy`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetSinkPolicy {
    _private: ()
}
impl GetSinkPolicy {
    /// Creates a new builder-style object to manufacture [`GetSinkPolicyInput`](crate::input::GetSinkPolicyInput).
    pub fn builder() -> crate::input::get_sink_policy_input::Builder {
        crate::input::get_sink_policy_input::Builder::default()
    }
    /// Creates a new `GetSinkPolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetSinkPolicy {
                type Output = std::result::Result<crate::output::GetSinkPolicyOutput, crate::error::GetSinkPolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_sink_policy_error(response)
                     } else {
                        crate::operation_deser::parse_get_sink_policy_response(response)
                     }
                }
            }

/// Operation shape for `ListAttachedLinks`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_attached_links`](crate::client::Client::list_attached_links).
            ///
            /// See [`crate::client::fluent_builders::ListAttachedLinks`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListAttachedLinks {
    _private: ()
}
impl ListAttachedLinks {
    /// Creates a new builder-style object to manufacture [`ListAttachedLinksInput`](crate::input::ListAttachedLinksInput).
    pub fn builder() -> crate::input::list_attached_links_input::Builder {
        crate::input::list_attached_links_input::Builder::default()
    }
    /// Creates a new `ListAttachedLinks` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListAttachedLinks {
                type Output = std::result::Result<crate::output::ListAttachedLinksOutput, crate::error::ListAttachedLinksError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_attached_links_error(response)
                     } else {
                        crate::operation_deser::parse_list_attached_links_response(response)
                     }
                }
            }

/// Operation shape for `ListLinks`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_links`](crate::client::Client::list_links).
            ///
            /// See [`crate::client::fluent_builders::ListLinks`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListLinks {
    _private: ()
}
impl ListLinks {
    /// Creates a new builder-style object to manufacture [`ListLinksInput`](crate::input::ListLinksInput).
    pub fn builder() -> crate::input::list_links_input::Builder {
        crate::input::list_links_input::Builder::default()
    }
    /// Creates a new `ListLinks` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListLinks {
                type Output = std::result::Result<crate::output::ListLinksOutput, crate::error::ListLinksError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_links_error(response)
                     } else {
                        crate::operation_deser::parse_list_links_response(response)
                     }
                }
            }

/// Operation shape for `ListSinks`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_sinks`](crate::client::Client::list_sinks).
            ///
            /// See [`crate::client::fluent_builders::ListSinks`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListSinks {
    _private: ()
}
impl ListSinks {
    /// Creates a new builder-style object to manufacture [`ListSinksInput`](crate::input::ListSinksInput).
    pub fn builder() -> crate::input::list_sinks_input::Builder {
        crate::input::list_sinks_input::Builder::default()
    }
    /// Creates a new `ListSinks` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListSinks {
                type Output = std::result::Result<crate::output::ListSinksOutput, crate::error::ListSinksError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_sinks_error(response)
                     } else {
                        crate::operation_deser::parse_list_sinks_response(response)
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

/// Operation shape for `PutSinkPolicy`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_sink_policy`](crate::client::Client::put_sink_policy).
            ///
            /// See [`crate::client::fluent_builders::PutSinkPolicy`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutSinkPolicy {
    _private: ()
}
impl PutSinkPolicy {
    /// Creates a new builder-style object to manufacture [`PutSinkPolicyInput`](crate::input::PutSinkPolicyInput).
    pub fn builder() -> crate::input::put_sink_policy_input::Builder {
        crate::input::put_sink_policy_input::Builder::default()
    }
    /// Creates a new `PutSinkPolicy` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutSinkPolicy {
                type Output = std::result::Result<crate::output::PutSinkPolicyOutput, crate::error::PutSinkPolicyError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_put_sink_policy_error(response)
                     } else {
                        crate::operation_deser::parse_put_sink_policy_response(response)
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

/// Operation shape for `UpdateLink`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_link`](crate::client::Client::update_link).
            ///
            /// See [`crate::client::fluent_builders::UpdateLink`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateLink {
    _private: ()
}
impl UpdateLink {
    /// Creates a new builder-style object to manufacture [`UpdateLinkInput`](crate::input::UpdateLinkInput).
    pub fn builder() -> crate::input::update_link_input::Builder {
        crate::input::update_link_input::Builder::default()
    }
    /// Creates a new `UpdateLink` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateLink {
                type Output = std::result::Result<crate::output::UpdateLinkOutput, crate::error::UpdateLinkError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_link_error(response)
                     } else {
                        crate::operation_deser::parse_update_link_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;


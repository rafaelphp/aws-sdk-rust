// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateGroup`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_group`](crate::client::Client::create_group).
            ///
            /// See [`crate::client::fluent_builders::CreateGroup`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateGroup {
    _private: ()
}
impl CreateGroup {
    /// Creates a new builder-style object to manufacture [`CreateGroupInput`](crate::input::CreateGroupInput).
    pub fn builder() -> crate::input::create_group_input::Builder {
        crate::input::create_group_input::Builder::default()
    }
    /// Creates a new `CreateGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateGroup {
                type Output = std::result::Result<crate::output::CreateGroupOutput, crate::error::CreateGroupError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_group_error(response)
                     } else {
                        crate::operation_deser::parse_create_group_response(response)
                     }
                }
            }

/// Operation shape for `DeleteGroup`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`delete_group`](crate::client::Client::delete_group).
            ///
            /// See [`crate::client::fluent_builders::DeleteGroup`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteGroup {
    _private: ()
}
impl DeleteGroup {
    /// Creates a new builder-style object to manufacture [`DeleteGroupInput`](crate::input::DeleteGroupInput).
    pub fn builder() -> crate::input::delete_group_input::Builder {
        crate::input::delete_group_input::Builder::default()
    }
    /// Creates a new `DeleteGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteGroup {
                type Output = std::result::Result<crate::output::DeleteGroupOutput, crate::error::DeleteGroupError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_delete_group_error(response)
                     } else {
                        crate::operation_deser::parse_delete_group_response(response)
                     }
                }
            }

/// Operation shape for `GetAccountSettings`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_account_settings`](crate::client::Client::get_account_settings).
            ///
            /// See [`crate::client::fluent_builders::GetAccountSettings`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetAccountSettings {
    _private: ()
}
impl GetAccountSettings {
    /// Creates a new builder-style object to manufacture [`GetAccountSettingsInput`](crate::input::GetAccountSettingsInput).
    pub fn builder() -> crate::input::get_account_settings_input::Builder {
        crate::input::get_account_settings_input::Builder::default()
    }
    /// Creates a new `GetAccountSettings` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetAccountSettings {
                type Output = std::result::Result<crate::output::GetAccountSettingsOutput, crate::error::GetAccountSettingsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_account_settings_error(response)
                     } else {
                        crate::operation_deser::parse_get_account_settings_response(response)
                     }
                }
            }

/// Operation shape for `GetGroup`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_group`](crate::client::Client::get_group).
            ///
            /// See [`crate::client::fluent_builders::GetGroup`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetGroup {
    _private: ()
}
impl GetGroup {
    /// Creates a new builder-style object to manufacture [`GetGroupInput`](crate::input::GetGroupInput).
    pub fn builder() -> crate::input::get_group_input::Builder {
        crate::input::get_group_input::Builder::default()
    }
    /// Creates a new `GetGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetGroup {
                type Output = std::result::Result<crate::output::GetGroupOutput, crate::error::GetGroupError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_group_error(response)
                     } else {
                        crate::operation_deser::parse_get_group_response(response)
                     }
                }
            }

/// Operation shape for `GetGroupConfiguration`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_group_configuration`](crate::client::Client::get_group_configuration).
            ///
            /// See [`crate::client::fluent_builders::GetGroupConfiguration`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetGroupConfiguration {
    _private: ()
}
impl GetGroupConfiguration {
    /// Creates a new builder-style object to manufacture [`GetGroupConfigurationInput`](crate::input::GetGroupConfigurationInput).
    pub fn builder() -> crate::input::get_group_configuration_input::Builder {
        crate::input::get_group_configuration_input::Builder::default()
    }
    /// Creates a new `GetGroupConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetGroupConfiguration {
                type Output = std::result::Result<crate::output::GetGroupConfigurationOutput, crate::error::GetGroupConfigurationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_group_configuration_error(response)
                     } else {
                        crate::operation_deser::parse_get_group_configuration_response(response)
                     }
                }
            }

/// Operation shape for `GetGroupQuery`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_group_query`](crate::client::Client::get_group_query).
            ///
            /// See [`crate::client::fluent_builders::GetGroupQuery`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetGroupQuery {
    _private: ()
}
impl GetGroupQuery {
    /// Creates a new builder-style object to manufacture [`GetGroupQueryInput`](crate::input::GetGroupQueryInput).
    pub fn builder() -> crate::input::get_group_query_input::Builder {
        crate::input::get_group_query_input::Builder::default()
    }
    /// Creates a new `GetGroupQuery` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetGroupQuery {
                type Output = std::result::Result<crate::output::GetGroupQueryOutput, crate::error::GetGroupQueryError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_group_query_error(response)
                     } else {
                        crate::operation_deser::parse_get_group_query_response(response)
                     }
                }
            }

/// Operation shape for `GetTags`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`get_tags`](crate::client::Client::get_tags).
            ///
            /// See [`crate::client::fluent_builders::GetTags`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetTags {
    _private: ()
}
impl GetTags {
    /// Creates a new builder-style object to manufacture [`GetTagsInput`](crate::input::GetTagsInput).
    pub fn builder() -> crate::input::get_tags_input::Builder {
        crate::input::get_tags_input::Builder::default()
    }
    /// Creates a new `GetTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetTags {
                type Output = std::result::Result<crate::output::GetTagsOutput, crate::error::GetTagsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_get_tags_error(response)
                     } else {
                        crate::operation_deser::parse_get_tags_response(response)
                     }
                }
            }

/// Operation shape for `GroupResources`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`group_resources`](crate::client::Client::group_resources).
            ///
            /// See [`crate::client::fluent_builders::GroupResources`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GroupResources {
    _private: ()
}
impl GroupResources {
    /// Creates a new builder-style object to manufacture [`GroupResourcesInput`](crate::input::GroupResourcesInput).
    pub fn builder() -> crate::input::group_resources_input::Builder {
        crate::input::group_resources_input::Builder::default()
    }
    /// Creates a new `GroupResources` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GroupResources {
                type Output = std::result::Result<crate::output::GroupResourcesOutput, crate::error::GroupResourcesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_group_resources_error(response)
                     } else {
                        crate::operation_deser::parse_group_resources_response(response)
                     }
                }
            }

/// Operation shape for `ListGroupResources`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_group_resources`](crate::client::Client::list_group_resources).
            ///
            /// See [`crate::client::fluent_builders::ListGroupResources`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListGroupResources {
    _private: ()
}
impl ListGroupResources {
    /// Creates a new builder-style object to manufacture [`ListGroupResourcesInput`](crate::input::ListGroupResourcesInput).
    pub fn builder() -> crate::input::list_group_resources_input::Builder {
        crate::input::list_group_resources_input::Builder::default()
    }
    /// Creates a new `ListGroupResources` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListGroupResources {
                type Output = std::result::Result<crate::output::ListGroupResourcesOutput, crate::error::ListGroupResourcesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_group_resources_error(response)
                     } else {
                        crate::operation_deser::parse_list_group_resources_response(response)
                     }
                }
            }

/// Operation shape for `ListGroups`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`list_groups`](crate::client::Client::list_groups).
            ///
            /// See [`crate::client::fluent_builders::ListGroups`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListGroups {
    _private: ()
}
impl ListGroups {
    /// Creates a new builder-style object to manufacture [`ListGroupsInput`](crate::input::ListGroupsInput).
    pub fn builder() -> crate::input::list_groups_input::Builder {
        crate::input::list_groups_input::Builder::default()
    }
    /// Creates a new `ListGroups` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListGroups {
                type Output = std::result::Result<crate::output::ListGroupsOutput, crate::error::ListGroupsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_list_groups_error(response)
                     } else {
                        crate::operation_deser::parse_list_groups_response(response)
                     }
                }
            }

/// Operation shape for `PutGroupConfiguration`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`put_group_configuration`](crate::client::Client::put_group_configuration).
            ///
            /// See [`crate::client::fluent_builders::PutGroupConfiguration`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutGroupConfiguration {
    _private: ()
}
impl PutGroupConfiguration {
    /// Creates a new builder-style object to manufacture [`PutGroupConfigurationInput`](crate::input::PutGroupConfigurationInput).
    pub fn builder() -> crate::input::put_group_configuration_input::Builder {
        crate::input::put_group_configuration_input::Builder::default()
    }
    /// Creates a new `PutGroupConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutGroupConfiguration {
                type Output = std::result::Result<crate::output::PutGroupConfigurationOutput, crate::error::PutGroupConfigurationError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 202 {
                        crate::operation_deser::parse_put_group_configuration_error(response)
                     } else {
                        crate::operation_deser::parse_put_group_configuration_response(response)
                     }
                }
            }

/// Operation shape for `SearchResources`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`search_resources`](crate::client::Client::search_resources).
            ///
            /// See [`crate::client::fluent_builders::SearchResources`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct SearchResources {
    _private: ()
}
impl SearchResources {
    /// Creates a new builder-style object to manufacture [`SearchResourcesInput`](crate::input::SearchResourcesInput).
    pub fn builder() -> crate::input::search_resources_input::Builder {
        crate::input::search_resources_input::Builder::default()
    }
    /// Creates a new `SearchResources` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SearchResources {
                type Output = std::result::Result<crate::output::SearchResourcesOutput, crate::error::SearchResourcesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_search_resources_error(response)
                     } else {
                        crate::operation_deser::parse_search_resources_response(response)
                     }
                }
            }

/// Operation shape for `Tag`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`tag`](crate::client::Client::tag).
            ///
            /// See [`crate::client::fluent_builders::Tag`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct Tag {
    _private: ()
}
impl Tag {
    /// Creates a new builder-style object to manufacture [`TagInput`](crate::input::TagInput).
    pub fn builder() -> crate::input::tag_input::Builder {
        crate::input::tag_input::Builder::default()
    }
    /// Creates a new `Tag` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for Tag {
                type Output = std::result::Result<crate::output::TagOutput, crate::error::TagError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_tag_error(response)
                     } else {
                        crate::operation_deser::parse_tag_response(response)
                     }
                }
            }

/// Operation shape for `UngroupResources`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`ungroup_resources`](crate::client::Client::ungroup_resources).
            ///
            /// See [`crate::client::fluent_builders::UngroupResources`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UngroupResources {
    _private: ()
}
impl UngroupResources {
    /// Creates a new builder-style object to manufacture [`UngroupResourcesInput`](crate::input::UngroupResourcesInput).
    pub fn builder() -> crate::input::ungroup_resources_input::Builder {
        crate::input::ungroup_resources_input::Builder::default()
    }
    /// Creates a new `UngroupResources` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UngroupResources {
                type Output = std::result::Result<crate::output::UngroupResourcesOutput, crate::error::UngroupResourcesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_ungroup_resources_error(response)
                     } else {
                        crate::operation_deser::parse_ungroup_resources_response(response)
                     }
                }
            }

/// Operation shape for `Untag`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`untag`](crate::client::Client::untag).
            ///
            /// See [`crate::client::fluent_builders::Untag`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct Untag {
    _private: ()
}
impl Untag {
    /// Creates a new builder-style object to manufacture [`UntagInput`](crate::input::UntagInput).
    pub fn builder() -> crate::input::untag_input::Builder {
        crate::input::untag_input::Builder::default()
    }
    /// Creates a new `Untag` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for Untag {
                type Output = std::result::Result<crate::output::UntagOutput, crate::error::UntagError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_untag_error(response)
                     } else {
                        crate::operation_deser::parse_untag_response(response)
                     }
                }
            }

/// Operation shape for `UpdateAccountSettings`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_account_settings`](crate::client::Client::update_account_settings).
            ///
            /// See [`crate::client::fluent_builders::UpdateAccountSettings`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateAccountSettings {
    _private: ()
}
impl UpdateAccountSettings {
    /// Creates a new builder-style object to manufacture [`UpdateAccountSettingsInput`](crate::input::UpdateAccountSettingsInput).
    pub fn builder() -> crate::input::update_account_settings_input::Builder {
        crate::input::update_account_settings_input::Builder::default()
    }
    /// Creates a new `UpdateAccountSettings` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateAccountSettings {
                type Output = std::result::Result<crate::output::UpdateAccountSettingsOutput, crate::error::UpdateAccountSettingsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_account_settings_error(response)
                     } else {
                        crate::operation_deser::parse_update_account_settings_response(response)
                     }
                }
            }

/// Operation shape for `UpdateGroup`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_group`](crate::client::Client::update_group).
            ///
            /// See [`crate::client::fluent_builders::UpdateGroup`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateGroup {
    _private: ()
}
impl UpdateGroup {
    /// Creates a new builder-style object to manufacture [`UpdateGroupInput`](crate::input::UpdateGroupInput).
    pub fn builder() -> crate::input::update_group_input::Builder {
        crate::input::update_group_input::Builder::default()
    }
    /// Creates a new `UpdateGroup` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateGroup {
                type Output = std::result::Result<crate::output::UpdateGroupOutput, crate::error::UpdateGroupError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_group_error(response)
                     } else {
                        crate::operation_deser::parse_update_group_response(response)
                     }
                }
            }

/// Operation shape for `UpdateGroupQuery`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`update_group_query`](crate::client::Client::update_group_query).
            ///
            /// See [`crate::client::fluent_builders::UpdateGroupQuery`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateGroupQuery {
    _private: ()
}
impl UpdateGroupQuery {
    /// Creates a new builder-style object to manufacture [`UpdateGroupQueryInput`](crate::input::UpdateGroupQueryInput).
    pub fn builder() -> crate::input::update_group_query_input::Builder {
        crate::input::update_group_query_input::Builder::default()
    }
    /// Creates a new `UpdateGroupQuery` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateGroupQuery {
                type Output = std::result::Result<crate::output::UpdateGroupQueryOutput, crate::error::UpdateGroupQueryError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_update_group_query_error(response)
                     } else {
                        crate::operation_deser::parse_update_group_query_response(response)
                     }
                }
            }

/// Operation customization and supporting types
pub mod customize;


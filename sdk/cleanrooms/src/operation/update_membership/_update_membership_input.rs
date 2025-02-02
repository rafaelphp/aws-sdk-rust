// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateMembershipInput {
    /// <p>The unique identifier of the membership.</p>
    pub membership_identifier: ::std::option::Option<::std::string::String>,
    /// <p>An indicator as to whether query logging has been enabled or disabled for the collaboration.</p>
    pub query_log_status: ::std::option::Option<crate::types::MembershipQueryLogStatus>,
}
impl UpdateMembershipInput {
    /// <p>The unique identifier of the membership.</p>
    pub fn membership_identifier(&self) -> ::std::option::Option<&str> {
        self.membership_identifier.as_deref()
    }
    /// <p>An indicator as to whether query logging has been enabled or disabled for the collaboration.</p>
    pub fn query_log_status(&self) -> ::std::option::Option<&crate::types::MembershipQueryLogStatus> {
        self.query_log_status.as_ref()
    }
}
impl UpdateMembershipInput {
    /// Creates a new builder-style object to manufacture [`UpdateMembershipInput`](crate::operation::update_membership::UpdateMembershipInput).
    pub fn builder() -> crate::operation::update_membership::builders::UpdateMembershipInputBuilder {
        crate::operation::update_membership::builders::UpdateMembershipInputBuilder::default()
    }
}

/// A builder for [`UpdateMembershipInput`](crate::operation::update_membership::UpdateMembershipInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UpdateMembershipInputBuilder {
    pub(crate) membership_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) query_log_status: ::std::option::Option<crate::types::MembershipQueryLogStatus>,
}
impl UpdateMembershipInputBuilder {
    /// <p>The unique identifier of the membership.</p>
    pub fn membership_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.membership_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the membership.</p>
    pub fn set_membership_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.membership_identifier = input;
        self
    }
    /// <p>The unique identifier of the membership.</p>
    pub fn get_membership_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.membership_identifier
    }
    /// <p>An indicator as to whether query logging has been enabled or disabled for the collaboration.</p>
    pub fn query_log_status(mut self, input: crate::types::MembershipQueryLogStatus) -> Self {
        self.query_log_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>An indicator as to whether query logging has been enabled or disabled for the collaboration.</p>
    pub fn set_query_log_status(mut self, input: ::std::option::Option<crate::types::MembershipQueryLogStatus>) -> Self {
        self.query_log_status = input;
        self
    }
    /// <p>An indicator as to whether query logging has been enabled or disabled for the collaboration.</p>
    pub fn get_query_log_status(&self) -> &::std::option::Option<crate::types::MembershipQueryLogStatus> {
        &self.query_log_status
    }
    /// Consumes the builder and constructs a [`UpdateMembershipInput`](crate::operation::update_membership::UpdateMembershipInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::update_membership::UpdateMembershipInput, ::aws_smithy_http::operation::error::BuildError> {
        ::std::result::Result::Ok(crate::operation::update_membership::UpdateMembershipInput {
            membership_identifier: self.membership_identifier,
            query_log_status: self.query_log_status,
        })
    }
}

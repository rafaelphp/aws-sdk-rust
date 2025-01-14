// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about an action for a request for which an authorization decision is made.</p>
/// <p>This data type is used as an request parameter to the <a href="https://docs.aws.amazon.com/verifiedpermissions/latest/apireference/API_IsAuthorized.html">IsAuthorized</a> and <a href="https://docs.aws.amazon.com/verifiedpermissions/latest/apireference/API_IsAuthorizedWithToken.html">IsAuthorizedWithToken</a> operations.</p>
/// <p>Example: <code>{ "actionId": "&lt;action name&gt;", "actionType": "Action" }</code> </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ActionIdentifier {
    /// <p>The type of an action.</p>
    pub action_type: ::std::option::Option<::std::string::String>,
    /// <p>The ID of an action.</p>
    pub action_id: ::std::option::Option<::std::string::String>,
}
impl ActionIdentifier {
    /// <p>The type of an action.</p>
    pub fn action_type(&self) -> ::std::option::Option<&str> {
        self.action_type.as_deref()
    }
    /// <p>The ID of an action.</p>
    pub fn action_id(&self) -> ::std::option::Option<&str> {
        self.action_id.as_deref()
    }
}
impl ActionIdentifier {
    /// Creates a new builder-style object to manufacture [`ActionIdentifier`](crate::types::ActionIdentifier).
    pub fn builder() -> crate::types::builders::ActionIdentifierBuilder {
        crate::types::builders::ActionIdentifierBuilder::default()
    }
}

/// A builder for [`ActionIdentifier`](crate::types::ActionIdentifier).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ActionIdentifierBuilder {
    pub(crate) action_type: ::std::option::Option<::std::string::String>,
    pub(crate) action_id: ::std::option::Option<::std::string::String>,
}
impl ActionIdentifierBuilder {
    /// <p>The type of an action.</p>
    pub fn action_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.action_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The type of an action.</p>
    pub fn set_action_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.action_type = input;
        self
    }
    /// <p>The type of an action.</p>
    pub fn get_action_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.action_type
    }
    /// <p>The ID of an action.</p>
    pub fn action_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.action_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of an action.</p>
    pub fn set_action_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.action_id = input;
        self
    }
    /// <p>The ID of an action.</p>
    pub fn get_action_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.action_id
    }
    /// Consumes the builder and constructs a [`ActionIdentifier`](crate::types::ActionIdentifier).
    pub fn build(self) -> crate::types::ActionIdentifier {
        crate::types::ActionIdentifier {
            action_type: self.action_type,
            action_id: self.action_id,
        }
    }
}

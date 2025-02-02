// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetPolicyTemplateOutput {
    /// <p>The ID of the policy store that contains the policy template.</p>
    pub policy_store_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the policy template.</p>
    pub policy_template_id: ::std::option::Option<::std::string::String>,
    /// <p>The description of the policy template.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The content of the body of the policy template written in the Cedar policy language.</p>
    pub statement: ::std::option::Option<::std::string::String>,
    /// <p>The date and time that the policy template was originally created.</p>
    pub created_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The date and time that the policy template was most recently updated.</p>
    pub last_updated_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl GetPolicyTemplateOutput {
    /// <p>The ID of the policy store that contains the policy template.</p>
    pub fn policy_store_id(&self) -> ::std::option::Option<&str> {
        self.policy_store_id.as_deref()
    }
    /// <p>The ID of the policy template.</p>
    pub fn policy_template_id(&self) -> ::std::option::Option<&str> {
        self.policy_template_id.as_deref()
    }
    /// <p>The description of the policy template.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The content of the body of the policy template written in the Cedar policy language.</p>
    pub fn statement(&self) -> ::std::option::Option<&str> {
        self.statement.as_deref()
    }
    /// <p>The date and time that the policy template was originally created.</p>
    pub fn created_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_date.as_ref()
    }
    /// <p>The date and time that the policy template was most recently updated.</p>
    pub fn last_updated_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_updated_date.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for GetPolicyTemplateOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetPolicyTemplateOutput {
    /// Creates a new builder-style object to manufacture [`GetPolicyTemplateOutput`](crate::operation::get_policy_template::GetPolicyTemplateOutput).
    pub fn builder() -> crate::operation::get_policy_template::builders::GetPolicyTemplateOutputBuilder {
        crate::operation::get_policy_template::builders::GetPolicyTemplateOutputBuilder::default()
    }
}

/// A builder for [`GetPolicyTemplateOutput`](crate::operation::get_policy_template::GetPolicyTemplateOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetPolicyTemplateOutputBuilder {
    pub(crate) policy_store_id: ::std::option::Option<::std::string::String>,
    pub(crate) policy_template_id: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) statement: ::std::option::Option<::std::string::String>,
    pub(crate) created_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_updated_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl GetPolicyTemplateOutputBuilder {
    /// <p>The ID of the policy store that contains the policy template.</p>
    pub fn policy_store_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy_store_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the policy store that contains the policy template.</p>
    pub fn set_policy_store_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy_store_id = input;
        self
    }
    /// <p>The ID of the policy store that contains the policy template.</p>
    pub fn get_policy_store_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.policy_store_id
    }
    /// <p>The ID of the policy template.</p>
    pub fn policy_template_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy_template_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the policy template.</p>
    pub fn set_policy_template_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy_template_id = input;
        self
    }
    /// <p>The ID of the policy template.</p>
    pub fn get_policy_template_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.policy_template_id
    }
    /// <p>The description of the policy template.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the policy template.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The description of the policy template.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// <p>The content of the body of the policy template written in the Cedar policy language.</p>
    pub fn statement(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.statement = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The content of the body of the policy template written in the Cedar policy language.</p>
    pub fn set_statement(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.statement = input;
        self
    }
    /// <p>The content of the body of the policy template written in the Cedar policy language.</p>
    pub fn get_statement(&self) -> &::std::option::Option<::std::string::String> {
        &self.statement
    }
    /// <p>The date and time that the policy template was originally created.</p>
    pub fn created_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time that the policy template was originally created.</p>
    pub fn set_created_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_date = input;
        self
    }
    /// <p>The date and time that the policy template was originally created.</p>
    pub fn get_created_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_date
    }
    /// <p>The date and time that the policy template was most recently updated.</p>
    pub fn last_updated_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_updated_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time that the policy template was most recently updated.</p>
    pub fn set_last_updated_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.last_updated_date = input;
        self
    }
    /// <p>The date and time that the policy template was most recently updated.</p>
    pub fn get_last_updated_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.last_updated_date
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetPolicyTemplateOutput`](crate::operation::get_policy_template::GetPolicyTemplateOutput).
    pub fn build(self) -> crate::operation::get_policy_template::GetPolicyTemplateOutput {
        crate::operation::get_policy_template::GetPolicyTemplateOutput {
            policy_store_id: self.policy_store_id,
            policy_template_id: self.policy_template_id,
            description: self.description,
            statement: self.statement,
            created_date: self.created_date,
            last_updated_date: self.last_updated_date,
            _request_id: self._request_id,
        }
    }
}

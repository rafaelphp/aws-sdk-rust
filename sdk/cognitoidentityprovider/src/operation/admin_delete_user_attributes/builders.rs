// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::admin_delete_user_attributes::_admin_delete_user_attributes_output::AdminDeleteUserAttributesOutputBuilder;

pub use crate::operation::admin_delete_user_attributes::_admin_delete_user_attributes_input::AdminDeleteUserAttributesInputBuilder;

impl AdminDeleteUserAttributesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::admin_delete_user_attributes::AdminDeleteUserAttributesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::admin_delete_user_attributes::AdminDeleteUserAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.admin_delete_user_attributes();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AdminDeleteUserAttributes`.
///
/// <p>Deletes the user attributes in a user pool as an administrator. Works on any user.</p>
/// <p>Calling this action requires developer credentials.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AdminDeleteUserAttributesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::admin_delete_user_attributes::builders::AdminDeleteUserAttributesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl AdminDeleteUserAttributesFluentBuilder {
    /// Creates a new `AdminDeleteUserAttributes`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AdminDeleteUserAttributes as a reference.
    pub fn as_input(&self) -> &crate::operation::admin_delete_user_attributes::builders::AdminDeleteUserAttributesInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::admin_delete_user_attributes::AdminDeleteUserAttributesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::admin_delete_user_attributes::AdminDeleteUserAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::admin_delete_user_attributes::AdminDeleteUserAttributes::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::admin_delete_user_attributes::AdminDeleteUserAttributes::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::admin_delete_user_attributes::AdminDeleteUserAttributesOutput,
            crate::operation::admin_delete_user_attributes::AdminDeleteUserAttributesError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::admin_delete_user_attributes::AdminDeleteUserAttributesError>,
    > {
        ::std::result::Result::Ok(crate::client::customize::orchestrator::CustomizableOperation {
            customizable_send: ::std::boxed::Box::new(move |config_override| {
                ::std::boxed::Box::pin(async { self.config_override(config_override).send().await })
            }),
            config_override: None,
            interceptors: vec![],
            runtime_plugins: vec![],
        })
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The user pool ID for the user pool where you want to delete user attributes.</p>
    pub fn user_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_pool_id(input.into());
        self
    }
    /// <p>The user pool ID for the user pool where you want to delete user attributes.</p>
    pub fn set_user_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_pool_id(input);
        self
    }
    /// <p>The user pool ID for the user pool where you want to delete user attributes.</p>
    pub fn get_user_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_user_pool_id()
    }
    /// <p>The user name of the user from which you would like to delete attributes.</p>
    pub fn username(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.username(input.into());
        self
    }
    /// <p>The user name of the user from which you would like to delete attributes.</p>
    pub fn set_username(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_username(input);
        self
    }
    /// <p>The user name of the user from which you would like to delete attributes.</p>
    pub fn get_username(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_username()
    }
    /// Appends an item to `UserAttributeNames`.
    ///
    /// To override the contents of this collection use [`set_user_attribute_names`](Self::set_user_attribute_names).
    ///
    /// <p>An array of strings representing the user attribute names you want to delete.</p>
    /// <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    pub fn user_attribute_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_attribute_names(input.into());
        self
    }
    /// <p>An array of strings representing the user attribute names you want to delete.</p>
    /// <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    pub fn set_user_attribute_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_user_attribute_names(input);
        self
    }
    /// <p>An array of strings representing the user attribute names you want to delete.</p>
    /// <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    pub fn get_user_attribute_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_user_attribute_names()
    }
}

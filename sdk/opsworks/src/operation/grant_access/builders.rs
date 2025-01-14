// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::grant_access::_grant_access_output::GrantAccessOutputBuilder;

pub use crate::operation::grant_access::_grant_access_input::GrantAccessInputBuilder;

impl GrantAccessInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::grant_access::GrantAccessOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::grant_access::GrantAccessError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.grant_access();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GrantAccess`.
///
/// <note>
/// <p>This action can be used only with Windows stacks.</p>
/// </note>
/// <p>Grants RDP access to a Windows instance for a specified time period.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GrantAccessFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::grant_access::builders::GrantAccessInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl GrantAccessFluentBuilder {
    /// Creates a new `GrantAccess`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GrantAccess as a reference.
    pub fn as_input(&self) -> &crate::operation::grant_access::builders::GrantAccessInputBuilder {
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
        crate::operation::grant_access::GrantAccessOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::grant_access::GrantAccessError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::grant_access::GrantAccess::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::grant_access::GrantAccess::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::grant_access::GrantAccessOutput,
            crate::operation::grant_access::GrantAccessError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::grant_access::GrantAccessError>,
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
    /// <p>The instance's AWS OpsWorks Stacks ID.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The instance's AWS OpsWorks Stacks ID.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The instance's AWS OpsWorks Stacks ID.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_id()
    }
    /// <p>The length of time (in minutes) that the grant is valid. When the grant expires at the end of this period, the user will no longer be able to use the credentials to log in. If the user is logged in at the time, he or she automatically will be logged out.</p>
    pub fn valid_for_in_minutes(mut self, input: i32) -> Self {
        self.inner = self.inner.valid_for_in_minutes(input);
        self
    }
    /// <p>The length of time (in minutes) that the grant is valid. When the grant expires at the end of this period, the user will no longer be able to use the credentials to log in. If the user is logged in at the time, he or she automatically will be logged out.</p>
    pub fn set_valid_for_in_minutes(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_valid_for_in_minutes(input);
        self
    }
    /// <p>The length of time (in minutes) that the grant is valid. When the grant expires at the end of this period, the user will no longer be able to use the credentials to log in. If the user is logged in at the time, he or she automatically will be logged out.</p>
    pub fn get_valid_for_in_minutes(&self) -> &::std::option::Option<i32> {
        self.inner.get_valid_for_in_minutes()
    }
}

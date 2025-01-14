// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::attach_policy::_attach_policy_output::AttachPolicyOutputBuilder;

pub use crate::operation::attach_policy::_attach_policy_input::AttachPolicyInputBuilder;

impl AttachPolicyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::attach_policy::AttachPolicyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::attach_policy::AttachPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.attach_policy();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AttachPolicy`.
///
/// <p>Attaches the specified policy to the specified principal (certificate or other credential).</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">AttachPolicy</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AttachPolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::attach_policy::builders::AttachPolicyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl AttachPolicyFluentBuilder {
    /// Creates a new `AttachPolicy`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AttachPolicy as a reference.
    pub fn as_input(&self) -> &crate::operation::attach_policy::builders::AttachPolicyInputBuilder {
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
        crate::operation::attach_policy::AttachPolicyOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::attach_policy::AttachPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::attach_policy::AttachPolicy::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::attach_policy::AttachPolicy::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::attach_policy::AttachPolicyOutput,
            crate::operation::attach_policy::AttachPolicyError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::attach_policy::AttachPolicyError>,
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
    /// <p>The name of the policy to attach.</p>
    pub fn policy_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy_name(input.into());
        self
    }
    /// <p>The name of the policy to attach.</p>
    pub fn set_policy_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy_name(input);
        self
    }
    /// <p>The name of the policy to attach.</p>
    pub fn get_policy_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_policy_name()
    }
    /// <p>The <a href="https://docs.aws.amazon.com/iot/latest/developerguide/security-iam.html">identity</a> to which the policy is attached. For example, a thing group or a certificate.</p>
    pub fn target(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target(input.into());
        self
    }
    /// <p>The <a href="https://docs.aws.amazon.com/iot/latest/developerguide/security-iam.html">identity</a> to which the policy is attached. For example, a thing group or a certificate.</p>
    pub fn set_target(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target(input);
        self
    }
    /// <p>The <a href="https://docs.aws.amazon.com/iot/latest/developerguide/security-iam.html">identity</a> to which the policy is attached. For example, a thing group or a certificate.</p>
    pub fn get_target(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target()
    }
}

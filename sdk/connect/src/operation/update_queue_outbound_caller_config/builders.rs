// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_queue_outbound_caller_config::_update_queue_outbound_caller_config_output::UpdateQueueOutboundCallerConfigOutputBuilder;

pub use crate::operation::update_queue_outbound_caller_config::_update_queue_outbound_caller_config_input::UpdateQueueOutboundCallerConfigInputBuilder;

impl UpdateQueueOutboundCallerConfigInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_queue_outbound_caller_config::UpdateQueueOutboundCallerConfigOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_queue_outbound_caller_config::UpdateQueueOutboundCallerConfigError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_queue_outbound_caller_config();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateQueueOutboundCallerConfig`.
///
/// <p>This API is in preview release for Amazon Connect and is subject to change.</p>
/// <p>Updates the outbound caller ID name, number, and outbound whisper flow for a specified queue.</p> <important>
/// <p>If the number being used in the input is claimed to a traffic distribution group, and you are calling this API using an instance in the Amazon Web Services Region where the traffic distribution group was created, you can use either a full phone number ARN or UUID value for the <code>OutboundCallerIdNumberId</code> value of the <a href="https://docs.aws.amazon.com/connect/latest/APIReference/API_OutboundCallerConfig">OutboundCallerConfig</a> request body parameter. However, if the number is claimed to a traffic distribution group and you are calling this API using an instance in the alternate Amazon Web Services Region associated with the traffic distribution group, you must provide a full phone number ARN. If a UUID is provided in this scenario, you will receive a <code>ResourceNotFoundException</code>.</p>
/// <p>Only use the phone number ARN format that doesn't contain <code>instance</code> in the path, for example, <code>arn:aws:connect:us-east-1:1234567890:phone-number/uuid</code>. This is the same ARN format that is returned when you call the <a href="https://docs.aws.amazon.com/connect/latest/APIReference/API_ListPhoneNumbersV2.html">ListPhoneNumbersV2</a> API.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateQueueOutboundCallerConfigFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_queue_outbound_caller_config::builders::UpdateQueueOutboundCallerConfigInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdateQueueOutboundCallerConfigFluentBuilder {
    /// Creates a new `UpdateQueueOutboundCallerConfig`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateQueueOutboundCallerConfig as a reference.
    pub fn as_input(&self) -> &crate::operation::update_queue_outbound_caller_config::builders::UpdateQueueOutboundCallerConfigInputBuilder {
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
        crate::operation::update_queue_outbound_caller_config::UpdateQueueOutboundCallerConfigOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_queue_outbound_caller_config::UpdateQueueOutboundCallerConfigError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_queue_outbound_caller_config::UpdateQueueOutboundCallerConfig::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_queue_outbound_caller_config::UpdateQueueOutboundCallerConfig::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_queue_outbound_caller_config::UpdateQueueOutboundCallerConfigOutput,
            crate::operation::update_queue_outbound_caller_config::UpdateQueueOutboundCallerConfigError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_queue_outbound_caller_config::UpdateQueueOutboundCallerConfigError>,
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
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_id()
    }
    /// <p>The identifier for the queue.</p>
    pub fn queue_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.queue_id(input.into());
        self
    }
    /// <p>The identifier for the queue.</p>
    pub fn set_queue_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_queue_id(input);
        self
    }
    /// <p>The identifier for the queue.</p>
    pub fn get_queue_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_queue_id()
    }
    /// <p>The outbound caller ID name, number, and outbound whisper flow.</p>
    pub fn outbound_caller_config(mut self, input: crate::types::OutboundCallerConfig) -> Self {
        self.inner = self.inner.outbound_caller_config(input);
        self
    }
    /// <p>The outbound caller ID name, number, and outbound whisper flow.</p>
    pub fn set_outbound_caller_config(mut self, input: ::std::option::Option<crate::types::OutboundCallerConfig>) -> Self {
        self.inner = self.inner.set_outbound_caller_config(input);
        self
    }
    /// <p>The outbound caller ID name, number, and outbound whisper flow.</p>
    pub fn get_outbound_caller_config(&self) -> &::std::option::Option<crate::types::OutboundCallerConfig> {
        self.inner.get_outbound_caller_config()
    }
}

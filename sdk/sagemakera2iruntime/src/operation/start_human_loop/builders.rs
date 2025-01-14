// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_human_loop::_start_human_loop_output::StartHumanLoopOutputBuilder;

pub use crate::operation::start_human_loop::_start_human_loop_input::StartHumanLoopInputBuilder;

impl StartHumanLoopInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_human_loop::StartHumanLoopOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_human_loop::StartHumanLoopError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_human_loop();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartHumanLoop`.
///
/// <p>Starts a human loop, provided that at least one activation condition is met.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartHumanLoopFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_human_loop::builders::StartHumanLoopInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl StartHumanLoopFluentBuilder {
    /// Creates a new `StartHumanLoop`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartHumanLoop as a reference.
    pub fn as_input(&self) -> &crate::operation::start_human_loop::builders::StartHumanLoopInputBuilder {
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
        crate::operation::start_human_loop::StartHumanLoopOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::start_human_loop::StartHumanLoopError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_human_loop::StartHumanLoop::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_human_loop::StartHumanLoop::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::start_human_loop::StartHumanLoopOutput,
            crate::operation::start_human_loop::StartHumanLoopError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::start_human_loop::StartHumanLoopError>,
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
    /// <p>The name of the human loop.</p>
    pub fn human_loop_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.human_loop_name(input.into());
        self
    }
    /// <p>The name of the human loop.</p>
    pub fn set_human_loop_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_human_loop_name(input);
        self
    }
    /// <p>The name of the human loop.</p>
    pub fn get_human_loop_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_human_loop_name()
    }
    /// <p>The Amazon Resource Name (ARN) of the flow definition associated with this human loop.</p>
    pub fn flow_definition_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.flow_definition_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the flow definition associated with this human loop.</p>
    pub fn set_flow_definition_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_flow_definition_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the flow definition associated with this human loop.</p>
    pub fn get_flow_definition_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_flow_definition_arn()
    }
    /// <p>An object that contains information about the human loop.</p>
    pub fn human_loop_input(mut self, input: crate::types::HumanLoopInput) -> Self {
        self.inner = self.inner.human_loop_input(input);
        self
    }
    /// <p>An object that contains information about the human loop.</p>
    pub fn set_human_loop_input(mut self, input: ::std::option::Option<crate::types::HumanLoopInput>) -> Self {
        self.inner = self.inner.set_human_loop_input(input);
        self
    }
    /// <p>An object that contains information about the human loop.</p>
    pub fn get_human_loop_input(&self) -> &::std::option::Option<crate::types::HumanLoopInput> {
        self.inner.get_human_loop_input()
    }
    /// <p>Attributes of the specified data. Use <code>DataAttributes</code> to specify if your data is free of personally identifiable information and/or free of adult content.</p>
    pub fn data_attributes(mut self, input: crate::types::HumanLoopDataAttributes) -> Self {
        self.inner = self.inner.data_attributes(input);
        self
    }
    /// <p>Attributes of the specified data. Use <code>DataAttributes</code> to specify if your data is free of personally identifiable information and/or free of adult content.</p>
    pub fn set_data_attributes(mut self, input: ::std::option::Option<crate::types::HumanLoopDataAttributes>) -> Self {
        self.inner = self.inner.set_data_attributes(input);
        self
    }
    /// <p>Attributes of the specified data. Use <code>DataAttributes</code> to specify if your data is free of personally identifiable information and/or free of adult content.</p>
    pub fn get_data_attributes(&self) -> &::std::option::Option<crate::types::HumanLoopDataAttributes> {
        self.inner.get_data_attributes()
    }
}

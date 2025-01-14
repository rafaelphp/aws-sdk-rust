// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_input::_update_input_output::UpdateInputOutputBuilder;

pub use crate::operation::update_input::_update_input_input::UpdateInputInputBuilder;

impl UpdateInputInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_input::UpdateInputOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_input::UpdateInputError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_input();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateInput`.
///
/// <p>Updates an input.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateInputFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_input::builders::UpdateInputInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdateInputFluentBuilder {
    /// Creates a new `UpdateInput`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateInput as a reference.
    pub fn as_input(&self) -> &crate::operation::update_input::builders::UpdateInputInputBuilder {
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
        crate::operation::update_input::UpdateInputOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_input::UpdateInputError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_input::UpdateInput::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_input::UpdateInput::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_input::UpdateInputOutput,
            crate::operation::update_input::UpdateInputError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_input::UpdateInputError>,
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
    /// <p>The name of the input you want to update.</p>
    pub fn input_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.input_name(input.into());
        self
    }
    /// <p>The name of the input you want to update.</p>
    pub fn set_input_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_input_name(input);
        self
    }
    /// <p>The name of the input you want to update.</p>
    pub fn get_input_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_input_name()
    }
    /// <p>A brief description of the input.</p>
    pub fn input_description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.input_description(input.into());
        self
    }
    /// <p>A brief description of the input.</p>
    pub fn set_input_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_input_description(input);
        self
    }
    /// <p>A brief description of the input.</p>
    pub fn get_input_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_input_description()
    }
    /// <p>The definition of the input.</p>
    pub fn input_definition(mut self, input: crate::types::InputDefinition) -> Self {
        self.inner = self.inner.input_definition(input);
        self
    }
    /// <p>The definition of the input.</p>
    pub fn set_input_definition(mut self, input: ::std::option::Option<crate::types::InputDefinition>) -> Self {
        self.inner = self.inner.set_input_definition(input);
        self
    }
    /// <p>The definition of the input.</p>
    pub fn get_input_definition(&self) -> &::std::option::Option<crate::types::InputDefinition> {
        self.inner.get_input_definition()
    }
}

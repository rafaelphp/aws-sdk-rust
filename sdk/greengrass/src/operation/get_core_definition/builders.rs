// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_core_definition::_get_core_definition_output::GetCoreDefinitionOutputBuilder;

pub use crate::operation::get_core_definition::_get_core_definition_input::GetCoreDefinitionInputBuilder;

impl GetCoreDefinitionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_core_definition::GetCoreDefinitionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_core_definition::GetCoreDefinitionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_core_definition();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetCoreDefinition`.
///
/// Retrieves information about a core definition version.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetCoreDefinitionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_core_definition::builders::GetCoreDefinitionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl GetCoreDefinitionFluentBuilder {
    /// Creates a new `GetCoreDefinition`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetCoreDefinition as a reference.
    pub fn as_input(&self) -> &crate::operation::get_core_definition::builders::GetCoreDefinitionInputBuilder {
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
        crate::operation::get_core_definition::GetCoreDefinitionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_core_definition::GetCoreDefinitionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_core_definition::GetCoreDefinition::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_core_definition::GetCoreDefinition::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::get_core_definition::GetCoreDefinitionOutput,
            crate::operation::get_core_definition::GetCoreDefinitionError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_core_definition::GetCoreDefinitionError>,
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
    /// The ID of the core definition.
    pub fn core_definition_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.core_definition_id(input.into());
        self
    }
    /// The ID of the core definition.
    pub fn set_core_definition_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_core_definition_id(input);
        self
    }
    /// The ID of the core definition.
    pub fn get_core_definition_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_core_definition_id()
    }
}

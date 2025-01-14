// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::import_documentation_parts::_import_documentation_parts_output::ImportDocumentationPartsOutputBuilder;

pub use crate::operation::import_documentation_parts::_import_documentation_parts_input::ImportDocumentationPartsInputBuilder;

impl ImportDocumentationPartsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::import_documentation_parts::ImportDocumentationPartsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::import_documentation_parts::ImportDocumentationPartsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.import_documentation_parts();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ImportDocumentationParts`.
///
/// <p>Imports documentation parts</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ImportDocumentationPartsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::import_documentation_parts::builders::ImportDocumentationPartsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl ImportDocumentationPartsFluentBuilder {
    /// Creates a new `ImportDocumentationParts`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ImportDocumentationParts as a reference.
    pub fn as_input(&self) -> &crate::operation::import_documentation_parts::builders::ImportDocumentationPartsInputBuilder {
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
        crate::operation::import_documentation_parts::ImportDocumentationPartsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::import_documentation_parts::ImportDocumentationPartsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::import_documentation_parts::ImportDocumentationParts::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::import_documentation_parts::ImportDocumentationParts::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::import_documentation_parts::ImportDocumentationPartsOutput,
            crate::operation::import_documentation_parts::ImportDocumentationPartsError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::import_documentation_parts::ImportDocumentationPartsError>,
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
    /// <p>The string identifier of the associated RestApi.</p>
    pub fn rest_api_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.rest_api_id(input.into());
        self
    }
    /// <p>The string identifier of the associated RestApi.</p>
    pub fn set_rest_api_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_rest_api_id(input);
        self
    }
    /// <p>The string identifier of the associated RestApi.</p>
    pub fn get_rest_api_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_rest_api_id()
    }
    /// <p>A query parameter to indicate whether to overwrite (<code>OVERWRITE</code>) any existing DocumentationParts definition or to merge (<code>MERGE</code>) the new definition into the existing one. The default value is <code>MERGE</code>.</p>
    pub fn mode(mut self, input: crate::types::PutMode) -> Self {
        self.inner = self.inner.mode(input);
        self
    }
    /// <p>A query parameter to indicate whether to overwrite (<code>OVERWRITE</code>) any existing DocumentationParts definition or to merge (<code>MERGE</code>) the new definition into the existing one. The default value is <code>MERGE</code>.</p>
    pub fn set_mode(mut self, input: ::std::option::Option<crate::types::PutMode>) -> Self {
        self.inner = self.inner.set_mode(input);
        self
    }
    /// <p>A query parameter to indicate whether to overwrite (<code>OVERWRITE</code>) any existing DocumentationParts definition or to merge (<code>MERGE</code>) the new definition into the existing one. The default value is <code>MERGE</code>.</p>
    pub fn get_mode(&self) -> &::std::option::Option<crate::types::PutMode> {
        self.inner.get_mode()
    }
    /// <p>A query parameter to specify whether to rollback the documentation importation (<code>true</code>) or not (<code>false</code>) when a warning is encountered. The default value is <code>false</code>.</p>
    pub fn fail_on_warnings(mut self, input: bool) -> Self {
        self.inner = self.inner.fail_on_warnings(input);
        self
    }
    /// <p>A query parameter to specify whether to rollback the documentation importation (<code>true</code>) or not (<code>false</code>) when a warning is encountered. The default value is <code>false</code>.</p>
    pub fn set_fail_on_warnings(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_fail_on_warnings(input);
        self
    }
    /// <p>A query parameter to specify whether to rollback the documentation importation (<code>true</code>) or not (<code>false</code>) when a warning is encountered. The default value is <code>false</code>.</p>
    pub fn get_fail_on_warnings(&self) -> &::std::option::Option<bool> {
        self.inner.get_fail_on_warnings()
    }
    /// <p>Raw byte array representing the to-be-imported documentation parts. To import from an OpenAPI file, this is a JSON object.</p>
    pub fn body(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.body(input);
        self
    }
    /// <p>Raw byte array representing the to-be-imported documentation parts. To import from an OpenAPI file, this is a JSON object.</p>
    pub fn set_body(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_body(input);
        self
    }
    /// <p>Raw byte array representing the to-be-imported documentation parts. To import from an OpenAPI file, this is a JSON object.</p>
    pub fn get_body(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_body()
    }
}

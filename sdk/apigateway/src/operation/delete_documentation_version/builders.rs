// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_documentation_version::_delete_documentation_version_output::DeleteDocumentationVersionOutputBuilder;

pub use crate::operation::delete_documentation_version::_delete_documentation_version_input::DeleteDocumentationVersionInputBuilder;

impl DeleteDocumentationVersionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_documentation_version::DeleteDocumentationVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_documentation_version::DeleteDocumentationVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_documentation_version();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteDocumentationVersion`.
///
/// <p>Deletes a documentation version.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteDocumentationVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_documentation_version::builders::DeleteDocumentationVersionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DeleteDocumentationVersionFluentBuilder {
    /// Creates a new `DeleteDocumentationVersion`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteDocumentationVersion as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_documentation_version::builders::DeleteDocumentationVersionInputBuilder {
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
        crate::operation::delete_documentation_version::DeleteDocumentationVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_documentation_version::DeleteDocumentationVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_documentation_version::DeleteDocumentationVersion::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_documentation_version::DeleteDocumentationVersion::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::delete_documentation_version::DeleteDocumentationVersionOutput,
            crate::operation::delete_documentation_version::DeleteDocumentationVersionError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::delete_documentation_version::DeleteDocumentationVersionError>,
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
    /// <p>The version identifier of a to-be-deleted documentation snapshot.</p>
    pub fn documentation_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.documentation_version(input.into());
        self
    }
    /// <p>The version identifier of a to-be-deleted documentation snapshot.</p>
    pub fn set_documentation_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_documentation_version(input);
        self
    }
    /// <p>The version identifier of a to-be-deleted documentation snapshot.</p>
    pub fn get_documentation_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_documentation_version()
    }
}

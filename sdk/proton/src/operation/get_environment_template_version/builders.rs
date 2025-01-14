// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_environment_template_version::_get_environment_template_version_output::GetEnvironmentTemplateVersionOutputBuilder;

pub use crate::operation::get_environment_template_version::_get_environment_template_version_input::GetEnvironmentTemplateVersionInputBuilder;

impl GetEnvironmentTemplateVersionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_environment_template_version::GetEnvironmentTemplateVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_environment_template_version::GetEnvironmentTemplateVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_environment_template_version();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetEnvironmentTemplateVersion`.
///
/// <p>Get detailed data for a major or minor version of an environment template.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetEnvironmentTemplateVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_environment_template_version::builders::GetEnvironmentTemplateVersionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl GetEnvironmentTemplateVersionFluentBuilder {
    /// Creates a new `GetEnvironmentTemplateVersion`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetEnvironmentTemplateVersion as a reference.
    pub fn as_input(&self) -> &crate::operation::get_environment_template_version::builders::GetEnvironmentTemplateVersionInputBuilder {
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
        crate::operation::get_environment_template_version::GetEnvironmentTemplateVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_environment_template_version::GetEnvironmentTemplateVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_environment_template_version::GetEnvironmentTemplateVersion::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_environment_template_version::GetEnvironmentTemplateVersion::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::get_environment_template_version::GetEnvironmentTemplateVersionOutput,
            crate::operation::get_environment_template_version::GetEnvironmentTemplateVersionError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_environment_template_version::GetEnvironmentTemplateVersionError>,
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
    /// <p>The name of the environment template a version of which you want to get detailed data for.</p>
    pub fn template_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.template_name(input.into());
        self
    }
    /// <p>The name of the environment template a version of which you want to get detailed data for.</p>
    pub fn set_template_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_template_name(input);
        self
    }
    /// <p>The name of the environment template a version of which you want to get detailed data for.</p>
    pub fn get_template_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_template_name()
    }
    /// <p>To get environment template major version detail data, include <code>major Version</code>.</p>
    pub fn major_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.major_version(input.into());
        self
    }
    /// <p>To get environment template major version detail data, include <code>major Version</code>.</p>
    pub fn set_major_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_major_version(input);
        self
    }
    /// <p>To get environment template major version detail data, include <code>major Version</code>.</p>
    pub fn get_major_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_major_version()
    }
    /// <p>To get environment template minor version detail data, include <code>minorVersion</code>.</p>
    pub fn minor_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.minor_version(input.into());
        self
    }
    /// <p>To get environment template minor version detail data, include <code>minorVersion</code>.</p>
    pub fn set_minor_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_minor_version(input);
        self
    }
    /// <p>To get environment template minor version detail data, include <code>minorVersion</code>.</p>
    pub fn get_minor_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_minor_version()
    }
}

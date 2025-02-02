// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_source_repository_clone_urls::_get_source_repository_clone_urls_output::GetSourceRepositoryCloneUrlsOutputBuilder;

pub use crate::operation::get_source_repository_clone_urls::_get_source_repository_clone_urls_input::GetSourceRepositoryCloneUrlsInputBuilder;

impl GetSourceRepositoryCloneUrlsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_source_repository_clone_urls::GetSourceRepositoryCloneUrlsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_source_repository_clone_urls::GetSourceRepositoryCloneUrlsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_source_repository_clone_urls();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetSourceRepositoryCloneUrls`.
///
/// <p>Returns information about the URLs that can be used with a Git client to clone a source repository.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetSourceRepositoryCloneUrlsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_source_repository_clone_urls::builders::GetSourceRepositoryCloneUrlsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl GetSourceRepositoryCloneUrlsFluentBuilder {
    /// Creates a new `GetSourceRepositoryCloneUrls`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetSourceRepositoryCloneUrls as a reference.
    pub fn as_input(&self) -> &crate::operation::get_source_repository_clone_urls::builders::GetSourceRepositoryCloneUrlsInputBuilder {
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
        crate::operation::get_source_repository_clone_urls::GetSourceRepositoryCloneUrlsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_source_repository_clone_urls::GetSourceRepositoryCloneUrlsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_source_repository_clone_urls::GetSourceRepositoryCloneUrls::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_source_repository_clone_urls::GetSourceRepositoryCloneUrls::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::get_source_repository_clone_urls::GetSourceRepositoryCloneUrlsOutput,
            crate::operation::get_source_repository_clone_urls::GetSourceRepositoryCloneUrlsError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_source_repository_clone_urls::GetSourceRepositoryCloneUrlsError>,
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
    /// <p>The name of the space.</p>
    pub fn space_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.space_name(input.into());
        self
    }
    /// <p>The name of the space.</p>
    pub fn set_space_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_space_name(input);
        self
    }
    /// <p>The name of the space.</p>
    pub fn get_space_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_space_name()
    }
    /// <p>The name of the project in the space.</p>
    pub fn project_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.project_name(input.into());
        self
    }
    /// <p>The name of the project in the space.</p>
    pub fn set_project_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_project_name(input);
        self
    }
    /// <p>The name of the project in the space.</p>
    pub fn get_project_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_project_name()
    }
    /// <p>The name of the source repository.</p>
    pub fn source_repository_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_repository_name(input.into());
        self
    }
    /// <p>The name of the source repository.</p>
    pub fn set_source_repository_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_repository_name(input);
        self
    }
    /// <p>The name of the source repository.</p>
    pub fn get_source_repository_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_repository_name()
    }
}

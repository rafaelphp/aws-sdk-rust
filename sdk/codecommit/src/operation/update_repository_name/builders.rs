// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_repository_name::_update_repository_name_output::UpdateRepositoryNameOutputBuilder;

pub use crate::operation::update_repository_name::_update_repository_name_input::UpdateRepositoryNameInputBuilder;

impl UpdateRepositoryNameInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_repository_name::UpdateRepositoryNameOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_repository_name::UpdateRepositoryNameError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_repository_name();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateRepositoryName`.
///
/// <p>Renames a repository. The repository name must be unique across the calling AWS account. Repository names are limited to 100 alphanumeric, dash, and underscore characters, and cannot include certain characters. The suffix .git is prohibited. For more information about the limits on repository names, see <a href="https://docs.aws.amazon.com/codecommit/latest/userguide/limits.html">Limits</a> in the AWS CodeCommit User Guide.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateRepositoryNameFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_repository_name::builders::UpdateRepositoryNameInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdateRepositoryNameFluentBuilder {
    /// Creates a new `UpdateRepositoryName`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateRepositoryName as a reference.
    pub fn as_input(&self) -> &crate::operation::update_repository_name::builders::UpdateRepositoryNameInputBuilder {
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
        crate::operation::update_repository_name::UpdateRepositoryNameOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_repository_name::UpdateRepositoryNameError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_repository_name::UpdateRepositoryName::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_repository_name::UpdateRepositoryName::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_repository_name::UpdateRepositoryNameOutput,
            crate::operation::update_repository_name::UpdateRepositoryNameError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_repository_name::UpdateRepositoryNameError>,
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
    /// <p>The current name of the repository.</p>
    pub fn old_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.old_name(input.into());
        self
    }
    /// <p>The current name of the repository.</p>
    pub fn set_old_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_old_name(input);
        self
    }
    /// <p>The current name of the repository.</p>
    pub fn get_old_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_old_name()
    }
    /// <p>The new name for the repository.</p>
    pub fn new_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.new_name(input.into());
        self
    }
    /// <p>The new name for the repository.</p>
    pub fn set_new_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_new_name(input);
        self
    }
    /// <p>The new name for the repository.</p>
    pub fn get_new_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_new_name()
    }
}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_project::_update_project_output::UpdateProjectOutputBuilder;

pub use crate::operation::update_project::_update_project_input::UpdateProjectInputBuilder;

impl UpdateProjectInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_project::UpdateProjectOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_project::UpdateProjectError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_project();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateProject`.
///
/// <p>Modifies the specified project name, given the project ARN and a new name.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateProjectFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_project::builders::UpdateProjectInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdateProjectFluentBuilder {
    /// Creates a new `UpdateProject`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateProject as a reference.
    pub fn as_input(&self) -> &crate::operation::update_project::builders::UpdateProjectInputBuilder {
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
        crate::operation::update_project::UpdateProjectOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_project::UpdateProjectError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_project::UpdateProject::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_project::UpdateProject::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_project::UpdateProjectOutput,
            crate::operation::update_project::UpdateProjectError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_project::UpdateProjectError>,
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
    /// <p>The Amazon Resource Name (ARN) of the project whose name to update.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the project whose name to update.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the project whose name to update.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_arn()
    }
    /// <p>A string that represents the new name of the project that you are updating.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>A string that represents the new name of the project that you are updating.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A string that represents the new name of the project that you are updating.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The number of minutes a test run in the project executes before it times out.</p>
    pub fn default_job_timeout_minutes(mut self, input: i32) -> Self {
        self.inner = self.inner.default_job_timeout_minutes(input);
        self
    }
    /// <p>The number of minutes a test run in the project executes before it times out.</p>
    pub fn set_default_job_timeout_minutes(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_default_job_timeout_minutes(input);
        self
    }
    /// <p>The number of minutes a test run in the project executes before it times out.</p>
    pub fn get_default_job_timeout_minutes(&self) -> &::std::option::Option<i32> {
        self.inner.get_default_job_timeout_minutes()
    }
    /// <p>The VPC security groups and subnets that are attached to a project.</p>
    pub fn vpc_config(mut self, input: crate::types::VpcConfig) -> Self {
        self.inner = self.inner.vpc_config(input);
        self
    }
    /// <p>The VPC security groups and subnets that are attached to a project.</p>
    pub fn set_vpc_config(mut self, input: ::std::option::Option<crate::types::VpcConfig>) -> Self {
        self.inner = self.inner.set_vpc_config(input);
        self
    }
    /// <p>The VPC security groups and subnets that are attached to a project.</p>
    pub fn get_vpc_config(&self) -> &::std::option::Option<crate::types::VpcConfig> {
        self.inner.get_vpc_config()
    }
}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_launch_configuration::_update_launch_configuration_output::UpdateLaunchConfigurationOutputBuilder;

pub use crate::operation::update_launch_configuration::_update_launch_configuration_input::UpdateLaunchConfigurationInputBuilder;

impl UpdateLaunchConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_launch_configuration::UpdateLaunchConfigurationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_launch_configuration::UpdateLaunchConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_launch_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateLaunchConfiguration`.
///
/// <p>Updates a LaunchConfiguration by Source Server ID.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateLaunchConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_launch_configuration::builders::UpdateLaunchConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdateLaunchConfigurationFluentBuilder {
    /// Creates a new `UpdateLaunchConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateLaunchConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::update_launch_configuration::builders::UpdateLaunchConfigurationInputBuilder {
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
        crate::operation::update_launch_configuration::UpdateLaunchConfigurationOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_launch_configuration::UpdateLaunchConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_launch_configuration::UpdateLaunchConfiguration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_launch_configuration::UpdateLaunchConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_launch_configuration::UpdateLaunchConfigurationOutput,
            crate::operation::update_launch_configuration::UpdateLaunchConfigurationError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_launch_configuration::UpdateLaunchConfigurationError>,
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
    /// <p>The ID of the Source Server that we want to retrieve a Launch Configuration for.</p>
    pub fn source_server_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_server_id(input.into());
        self
    }
    /// <p>The ID of the Source Server that we want to retrieve a Launch Configuration for.</p>
    pub fn set_source_server_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_server_id(input);
        self
    }
    /// <p>The ID of the Source Server that we want to retrieve a Launch Configuration for.</p>
    pub fn get_source_server_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_server_id()
    }
    /// <p>The name of the launch configuration.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the launch configuration.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the launch configuration.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The state of the Recovery Instance in EC2 after the recovery operation.</p>
    pub fn launch_disposition(mut self, input: crate::types::LaunchDisposition) -> Self {
        self.inner = self.inner.launch_disposition(input);
        self
    }
    /// <p>The state of the Recovery Instance in EC2 after the recovery operation.</p>
    pub fn set_launch_disposition(mut self, input: ::std::option::Option<crate::types::LaunchDisposition>) -> Self {
        self.inner = self.inner.set_launch_disposition(input);
        self
    }
    /// <p>The state of the Recovery Instance in EC2 after the recovery operation.</p>
    pub fn get_launch_disposition(&self) -> &::std::option::Option<crate::types::LaunchDisposition> {
        self.inner.get_launch_disposition()
    }
    /// <p>Whether Elastic Disaster Recovery should try to automatically choose the instance type that best matches the OS, CPU, and RAM of your Source Server.</p>
    pub fn target_instance_type_right_sizing_method(mut self, input: crate::types::TargetInstanceTypeRightSizingMethod) -> Self {
        self.inner = self.inner.target_instance_type_right_sizing_method(input);
        self
    }
    /// <p>Whether Elastic Disaster Recovery should try to automatically choose the instance type that best matches the OS, CPU, and RAM of your Source Server.</p>
    pub fn set_target_instance_type_right_sizing_method(
        mut self,
        input: ::std::option::Option<crate::types::TargetInstanceTypeRightSizingMethod>,
    ) -> Self {
        self.inner = self.inner.set_target_instance_type_right_sizing_method(input);
        self
    }
    /// <p>Whether Elastic Disaster Recovery should try to automatically choose the instance type that best matches the OS, CPU, and RAM of your Source Server.</p>
    pub fn get_target_instance_type_right_sizing_method(&self) -> &::std::option::Option<crate::types::TargetInstanceTypeRightSizingMethod> {
        self.inner.get_target_instance_type_right_sizing_method()
    }
    /// <p>Whether we should copy the Private IP of the Source Server to the Recovery Instance.</p>
    pub fn copy_private_ip(mut self, input: bool) -> Self {
        self.inner = self.inner.copy_private_ip(input);
        self
    }
    /// <p>Whether we should copy the Private IP of the Source Server to the Recovery Instance.</p>
    pub fn set_copy_private_ip(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_copy_private_ip(input);
        self
    }
    /// <p>Whether we should copy the Private IP of the Source Server to the Recovery Instance.</p>
    pub fn get_copy_private_ip(&self) -> &::std::option::Option<bool> {
        self.inner.get_copy_private_ip()
    }
    /// <p>Whether we want to copy the tags of the Source Server to the EC2 machine of the Recovery Instance.</p>
    pub fn copy_tags(mut self, input: bool) -> Self {
        self.inner = self.inner.copy_tags(input);
        self
    }
    /// <p>Whether we want to copy the tags of the Source Server to the EC2 machine of the Recovery Instance.</p>
    pub fn set_copy_tags(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_copy_tags(input);
        self
    }
    /// <p>Whether we want to copy the tags of the Source Server to the EC2 machine of the Recovery Instance.</p>
    pub fn get_copy_tags(&self) -> &::std::option::Option<bool> {
        self.inner.get_copy_tags()
    }
    /// <p>The licensing configuration to be used for this launch configuration.</p>
    pub fn licensing(mut self, input: crate::types::Licensing) -> Self {
        self.inner = self.inner.licensing(input);
        self
    }
    /// <p>The licensing configuration to be used for this launch configuration.</p>
    pub fn set_licensing(mut self, input: ::std::option::Option<crate::types::Licensing>) -> Self {
        self.inner = self.inner.set_licensing(input);
        self
    }
    /// <p>The licensing configuration to be used for this launch configuration.</p>
    pub fn get_licensing(&self) -> &::std::option::Option<crate::types::Licensing> {
        self.inner.get_licensing()
    }
}

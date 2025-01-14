// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_service::_delete_service_output::DeleteServiceOutputBuilder;

pub use crate::operation::delete_service::_delete_service_input::DeleteServiceInputBuilder;

impl DeleteServiceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_service::DeleteServiceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_service::DeleteServiceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_service();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteService`.
///
/// <p>Deletes an Amazon Web Services Migration Hub Refactor Spaces service. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteServiceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_service::builders::DeleteServiceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DeleteServiceFluentBuilder {
    /// Creates a new `DeleteService`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteService as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_service::builders::DeleteServiceInputBuilder {
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
        crate::operation::delete_service::DeleteServiceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::delete_service::DeleteServiceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_service::DeleteService::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_service::DeleteService::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::delete_service::DeleteServiceOutput,
            crate::operation::delete_service::DeleteServiceError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::delete_service::DeleteServiceError>,
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
    /// <p>The ID of the environment that the service is in.</p>
    pub fn environment_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.environment_identifier(input.into());
        self
    }
    /// <p>The ID of the environment that the service is in.</p>
    pub fn set_environment_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_environment_identifier(input);
        self
    }
    /// <p>The ID of the environment that the service is in.</p>
    pub fn get_environment_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_environment_identifier()
    }
    /// <p>Deletes a Refactor Spaces service.</p> <note>
    /// <p>The <code>RefactorSpacesSecurityGroup</code> security group must be removed from all Amazon Web Services resources in the virtual private cloud (VPC) prior to deleting a service with a URL endpoint in a VPC.</p>
    /// </note>
    pub fn application_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.application_identifier(input.into());
        self
    }
    /// <p>Deletes a Refactor Spaces service.</p> <note>
    /// <p>The <code>RefactorSpacesSecurityGroup</code> security group must be removed from all Amazon Web Services resources in the virtual private cloud (VPC) prior to deleting a service with a URL endpoint in a VPC.</p>
    /// </note>
    pub fn set_application_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_application_identifier(input);
        self
    }
    /// <p>Deletes a Refactor Spaces service.</p> <note>
    /// <p>The <code>RefactorSpacesSecurityGroup</code> security group must be removed from all Amazon Web Services resources in the virtual private cloud (VPC) prior to deleting a service with a URL endpoint in a VPC.</p>
    /// </note>
    pub fn get_application_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_application_identifier()
    }
    /// <p>The ID of the service to delete.</p>
    pub fn service_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_identifier(input.into());
        self
    }
    /// <p>The ID of the service to delete.</p>
    pub fn set_service_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_identifier(input);
        self
    }
    /// <p>The ID of the service to delete.</p>
    pub fn get_service_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_service_identifier()
    }
}

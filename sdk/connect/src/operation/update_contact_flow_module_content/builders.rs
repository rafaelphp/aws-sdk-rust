// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_contact_flow_module_content::_update_contact_flow_module_content_output::UpdateContactFlowModuleContentOutputBuilder;

pub use crate::operation::update_contact_flow_module_content::_update_contact_flow_module_content_input::UpdateContactFlowModuleContentInputBuilder;

impl UpdateContactFlowModuleContentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_contact_flow_module_content::UpdateContactFlowModuleContentOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_contact_flow_module_content::UpdateContactFlowModuleContentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_contact_flow_module_content();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateContactFlowModuleContent`.
///
/// <p>Updates specified flow module for the specified Amazon Connect instance. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateContactFlowModuleContentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_contact_flow_module_content::builders::UpdateContactFlowModuleContentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl UpdateContactFlowModuleContentFluentBuilder {
    /// Creates a new `UpdateContactFlowModuleContent`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateContactFlowModuleContent as a reference.
    pub fn as_input(&self) -> &crate::operation::update_contact_flow_module_content::builders::UpdateContactFlowModuleContentInputBuilder {
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
        crate::operation::update_contact_flow_module_content::UpdateContactFlowModuleContentOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_contact_flow_module_content::UpdateContactFlowModuleContentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_contact_flow_module_content::UpdateContactFlowModuleContent::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_contact_flow_module_content::UpdateContactFlowModuleContent::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::update_contact_flow_module_content::UpdateContactFlowModuleContentOutput,
            crate::operation::update_contact_flow_module_content::UpdateContactFlowModuleContentError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_contact_flow_module_content::UpdateContactFlowModuleContentError>,
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
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_id()
    }
    /// <p>The identifier of the flow module.</p>
    pub fn contact_flow_module_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.contact_flow_module_id(input.into());
        self
    }
    /// <p>The identifier of the flow module.</p>
    pub fn set_contact_flow_module_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_contact_flow_module_id(input);
        self
    }
    /// <p>The identifier of the flow module.</p>
    pub fn get_contact_flow_module_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_contact_flow_module_id()
    }
    /// <p>The content of the flow module.</p>
    pub fn content(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.content(input.into());
        self
    }
    /// <p>The content of the flow module.</p>
    pub fn set_content(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_content(input);
        self
    }
    /// <p>The content of the flow module.</p>
    pub fn get_content(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_content()
    }
}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_network_interface_attribute::_describe_network_interface_attribute_output::DescribeNetworkInterfaceAttributeOutputBuilder;

pub use crate::operation::describe_network_interface_attribute::_describe_network_interface_attribute_input::DescribeNetworkInterfaceAttributeInputBuilder;

impl DescribeNetworkInterfaceAttributeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_network_interface_attribute::DescribeNetworkInterfaceAttributeOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_network_interface_attribute::DescribeNetworkInterfaceAttributeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_network_interface_attribute();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeNetworkInterfaceAttribute`.
///
/// <p>Describes a network interface attribute. You can specify only one attribute at a time.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeNetworkInterfaceAttributeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_network_interface_attribute::builders::DescribeNetworkInterfaceAttributeInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DescribeNetworkInterfaceAttributeFluentBuilder {
    /// Creates a new `DescribeNetworkInterfaceAttribute`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeNetworkInterfaceAttribute as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_network_interface_attribute::builders::DescribeNetworkInterfaceAttributeInputBuilder {
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
        crate::operation::describe_network_interface_attribute::DescribeNetworkInterfaceAttributeOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_network_interface_attribute::DescribeNetworkInterfaceAttributeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_network_interface_attribute::DescribeNetworkInterfaceAttribute::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_network_interface_attribute::DescribeNetworkInterfaceAttribute::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::describe_network_interface_attribute::DescribeNetworkInterfaceAttributeOutput,
            crate::operation::describe_network_interface_attribute::DescribeNetworkInterfaceAttributeError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_network_interface_attribute::DescribeNetworkInterfaceAttributeError>,
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
    /// <p>The attribute of the network interface. This parameter is required.</p>
    pub fn attribute(mut self, input: crate::types::NetworkInterfaceAttribute) -> Self {
        self.inner = self.inner.attribute(input);
        self
    }
    /// <p>The attribute of the network interface. This parameter is required.</p>
    pub fn set_attribute(mut self, input: ::std::option::Option<crate::types::NetworkInterfaceAttribute>) -> Self {
        self.inner = self.inner.set_attribute(input);
        self
    }
    /// <p>The attribute of the network interface. This parameter is required.</p>
    pub fn get_attribute(&self) -> &::std::option::Option<crate::types::NetworkInterfaceAttribute> {
        self.inner.get_attribute()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    /// <p>The ID of the network interface.</p>
    pub fn network_interface_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.network_interface_id(input.into());
        self
    }
    /// <p>The ID of the network interface.</p>
    pub fn set_network_interface_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_network_interface_id(input);
        self
    }
    /// <p>The ID of the network interface.</p>
    pub fn get_network_interface_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_network_interface_id()
    }
}
